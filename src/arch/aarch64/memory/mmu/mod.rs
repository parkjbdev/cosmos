pub mod descriptors;
pub mod translation_table;

use crate::bsp::memory::symbols::PAGE_SIZE;
use crate::memory::types::*;
use crate::memory::{self, mmu::error::MMUEnableError};
use aarch64_cpu::registers::ID_AA64MMFR0_EL1;
use aarch64_cpu::{asm::barrier, registers::*};
use log::info;
use tock_registers::interfaces::{ReadWriteable, Readable};

pub(super) struct MemoryManagementUnit;
impl memory::mmu::interface::MMU for MemoryManagementUnit {
    fn init(&self, phys_table_baddr: Address<Physical>) -> Result<(), MMUEnableError> {
        // Check
        if self.is_enabled() {
            return Err(MMUEnableError::AlreadyEnabled);
        }
        let page_size = unsafe { PAGE_SIZE.get() } as usize;

        if !ID_AA64MMFR0_EL1.matches_all(match page_size {
            65536 => ID_AA64MMFR0_EL1::TGran64::Supported,
            16384 => ID_AA64MMFR0_EL1::TGran16::Supported,
            4096 => ID_AA64MMFR0_EL1::TGran4::Supported,
            _ => return Err(MMUEnableError::InvalidGranuleSize(page_size)),
        }) {
            return Err(MMUEnableError::GranuleNotSupported(page_size));
        }

        info!("ID_AA64MMFR0_EL1: {:#b}", ID_AA64MMFR0_EL1.get());

        let pa_range = match ID_AA64MMFR0_EL1.read(ID_AA64MMFR0_EL1::PARange) {
            0b0000 => 32,
            0b0001 => 36,
            0b0010 => 40,
            0b0011 => 42,
            0b0100 => 44,
            0b0101 => 48,
            0b0110 => 52,
            _ => 0,
        };

        info!("Physical Address Range: {}", pa_range);

        let asidbits = ID_AA64MMFR0_EL1.read(ID_AA64MMFR0_EL1::ASIDBits);
        info!(
            "ASID Bits: {}",
            match asidbits {
                0b0000 => 8,
                0b0010 => 16,
                _ => panic!("Invalid ASID Bits"),
            }
        );

        // Setup MAIR: Prepare the memory attribute indirection register
        MAIR_EL1.write(
            MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc
                + MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc
                + MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck,
        );

        // Set Kernel Table physical base address to TTBR0_EL1
        TTBR0_EL1.set_baddr(phys_table_baddr.value() as u64);

        // Configure Translation Control
        // Configure various settings of stage 1 of the EL1 translation regime.
        let t0sz = (64 - 5) as u64;
        TCR_EL1.write(
            TCR_EL1::TBI0::Used
                + TCR_EL1::IPS::Bits_40
                + TCR_EL1::SH0::Inner
                + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
                + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
                + TCR_EL1::EPD0::EnableTTBR0Walks
                + TCR_EL1::A1::TTBR0
                + TCR_EL1::T0SZ.val(t0sz)
                + TCR_EL1::EPD1::DisableTTBR1Walks,
        );

        barrier::isb(barrier::SY);

        // Enable the MMU and turn on data and instruction caching.
        SCTLR_EL1.modify(
            SCTLR_EL1::M::Enable // MMU enable for EL1 and EL0 stage 1 address translation.
            + SCTLR_EL1::C::Cacheable // Cacheability control, for data accesses.
            + SCTLR_EL1::I::Cacheable, // Instruction access Cacheability control, for accesses at EL0 and EL1
        );

        barrier::isb(barrier::SY);

        Ok(())
    }

    fn is_enabled(&self) -> bool {
        SCTLR_EL1.matches_all(SCTLR_EL1::M::Enable)
    }
}

pub(super) static MMU: MemoryManagementUnit = MemoryManagementUnit;

impl<const AS_SIZE: usize> memory::address_space::AddressSpace<AS_SIZE> {
    /// Checks for architectural restrictions.
    pub const fn arch_address_space_size_sanity_checks() {
        // Size must be at least one full 512 MiB table.
        assert!((AS_SIZE % Granule512MB::SIZE) == 0);

        // Check for 48 bit virtual address size as maximum, which is supported by any ARMv8
        // version.
        assert!(AS_SIZE <= (1 << 48));
    }
}

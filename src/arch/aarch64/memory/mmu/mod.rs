pub mod descriptors;
pub mod mair;
pub mod translation_table;

use crate::bsp::memory::symbols;
use crate::memory::types::*;
use crate::memory::{self, mmu::error::MMUEnableError};
use aarch64_cpu::{asm::barrier, registers::*};
use log::info;
use tock_registers::interfaces::{ReadWriteable, Readable};
// use crate::sync::null_lock::NullLock;
use core::arch::asm;

pub(super) static MMU: MemoryManagementUnit = MemoryManagementUnit;

pub(super) struct MemoryManagementUnit;
impl memory::mmu::interface::MMU for MemoryManagementUnit {
    fn init(&self, phys_table_baddr: Address<Physical>) -> Result<(), MMUEnableError> {
        // Check
        if self.is_enabled() {
            return Err(MMUEnableError::AlreadyEnabled);
        }

        // Setup MAIR: Prepare the memory attribute indirection register
        let attr0 = MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck;
        let attr1 = MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc
            + MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc;
        MAIR_EL1.write(attr0 + attr1);

        // Set Kernel Table physical base address to TTBR0_EL1
        TTBR0_EL1.set_baddr(phys_table_baddr.value() as u64);

        // let tt = unsafe {
        //     sptr::from_exposed_addr::<NullLock<KernelTranslationTable>>(
        //         (TTBR0_EL1.read(TTBR0_EL1::BADDR) << 1) as usize,
        //     )
        //     .as_ref()
        // }
        // .unwrap();

        // Configure Translation Control
        // Configure various settings of stage 1 of the EL1 translation regime.
        let t0sz = 16;

        // Enable TTBR0 and TTBR1 walks, page size = 4K, vaddr size = 48 bits, paddr size = 40 bits.
        // let tcr_flags0 = TCR_EL1::EPD0::EnableTTBR0Walks
        //     + TCR_EL1::TG0::KiB_64
        //     + TCR_EL1::SH0::Inner
        //     + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        //     + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        //     + TCR_EL1::T0SZ.val(16);
        // let tcr_flags1 = TCR_EL1::EPD1::EnableTTBR1Walks
        //     + TCR_EL1::TG1::KiB_64
        //     + TCR_EL1::SH1::Inner
        //     + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        //     + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        //     + TCR_EL1::T1SZ.val(16);

        // TCR_EL1.write(TCR_EL1::IPS::Bits_48 + tcr_flags0 + tcr_flags1);

        TCR_EL1.write(
            // 64 KiB granule
            TCR_EL1::TG0::KiB_64
                // Top Byte ignored
                + TCR_EL1::TBI0::Used
                // Intermediate Physical Address Size
                + TCR_EL1::IPS::Bits_48
                // + TCR_EL1::IPS::Bits_40
                // Sharability attribute
                + TCR_EL1::SH0::Inner
                // Inner Cacheability attribute
                + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
                // Outer Cacheability attribute
                + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
                + TCR_EL1::EPD0::EnableTTBR0Walks
                + TCR_EL1::EPD1::DisableTTBR1Walks
                + TCR_EL1::A1::TTBR0
                + TCR_EL1::T0SZ.val(t0sz)
                + TCR_EL1::HA::Enable // Allow the MMU to update the ACCESSED flag.
                + TCR_EL1::HD::Enable, // Allow the MMU to update the DIRTY flag.
        );

        barrier::dsb(barrier::ISHST);
        barrier::isb(barrier::SY);

        unsafe {
            asm!(
                "isb
                tlbi vmalle1
                ic iallu
                dsb nsh
                isb"
            );
        }
        __println!("Enabling MMU...");

        let sctlr_el1 = SCTLR_EL1::M::Enable + // MMU enable for EL1 and EL0 stage 1 address translation.
            SCTLR_EL1::A::Enable +
            SCTLR_EL1::C::Cacheable + // Cacheability control, for data accesses.
            SCTLR_EL1::I::Cacheable; // Instruction access Cacheability control, for accesses at EL0 and EL1

        // Enable the MMU and turn on data and instruction caching.
        SCTLR_EL1.modify(sctlr_el1);

        barrier::dsb(barrier::ISH);
        barrier::isb(barrier::SY);

        Ok(())
    }

    fn is_enabled(&self) -> bool {
        SCTLR_EL1.matches_all(SCTLR_EL1::M::Enable)
    }
}

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

pub fn print_stat() -> Result<(), MMUEnableError> {
    let page_size = symbols::page_size();

    if !ID_AA64MMFR0_EL1.matches_all(match page_size {
        65536 => ID_AA64MMFR0_EL1::TGran64::Supported,
        16384 => ID_AA64MMFR0_EL1::TGran16::Supported,
        4096 => ID_AA64MMFR0_EL1::TGran4::Supported,
        _ => return Err(MMUEnableError::InvalidGranuleSize(page_size)),
    }) {
        return Err(MMUEnableError::GranuleNotSupported(page_size));
    }

    info!("ID_AA64MMFR0_EL1: {:#b}", ID_AA64MMFR0_EL1.get());
    info!(
        "ID_AA64MMFR0_EL1::TGran: {}",
        match page_size {
            65536 => "ID_AA64MMFR0_EL1::TGran64::Supported",
            16384 => "ID_AA64MMFR0_EL1::TGran16::Supported",
            4096 => "ID_AA64MMFR0_EL1::TGran4::Supported",
            _ => return Err(MMUEnableError::InvalidGranuleSize(page_size)),
        }
    );

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

    Ok(())
}

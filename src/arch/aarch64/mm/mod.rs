use crate::{arch::dtb, utils::MemorySize};

use self::err::MMUEnableError;
use aarch64_cpu::{asm::barrier::*, registers::*};
use log::info;
use tock_registers::interfaces::{ReadWriteable, Readable};

use super::PAGE_SIZE;

mod descriptor;
mod err;
mod phys;
mod virt;

use descriptor::*;

pub fn init() {
    match MemoryManagementUnit.init() {
        Ok(_) => {}
        Err(e) => {
            panic!("Failed to enable MMU: {:?}", e);
        }
    };
}

struct MemoryManagementUnit;
impl MemoryManagementUnit {
    fn init(&self) -> Result<(), MMUEnableError> {
        // Check
        if self.is_enabled() {
            return Err(MMUEnableError::AlreadyEnabled);
        }
        let page_size = unsafe { PAGE_SIZE };

        if !ID_AA64MMFR0_EL1.matches_all(match page_size.0 {
            65536 => ID_AA64MMFR0_EL1::TGran64::Supported,
            16384 => ID_AA64MMFR0_EL1::TGran16::Supported,
            4096 => ID_AA64MMFR0_EL1::TGran4::Supported,
            _ => return Err(MMUEnableError::InvalidGranuleSize(page_size.0))
        }) {
            return Err(MMUEnableError::GranuleNotSupported(page_size.0));
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
            _ => 0
        };

        info!("Physical Address Range: {}", pa_range);

        // Setup MAIR: Prepare the memory attribute indirection register
        MAIR_EL1.write(
            MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc
                + MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc
                + MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck,
        );

        // TODO: Kernel Table Populate Translation Table entries

        // TODO: Set Kernel Table physical base address to TTBR0_EL1

        // TODO: Configure Translation Control
        // Configure various settings of stage 1 of the EL1 translation regime.

        isb(SY);

        // Enable the MMU and turn on data and instruction caching.
        // SCTLR_EL1.modify(
        //     SCTLR_EL1::M::Enable // MMU enable for EL1 and EL0 stage 1 address translation.
        //     + SCTLR_EL1::C::Cacheable // Cacheability control, for data accesses.
        //     + SCTLR_EL1::I::Cacheable, // Instruction access Cacheability control, for accesses at EL0 and EL1
        // );

        isb(SY);

        Ok(())
    }

    fn is_enabled(&self) -> bool {
        SCTLR_EL1.matches_all(SCTLR_EL1::M::Enable)
    }
}

pub fn get_page_size() -> MemorySize {
    unsafe { PAGE_SIZE }
}

pub fn get_ramrange() -> (u64, MemorySize) {
    let mem_devt = dtb::get_dtb()
        .get_property("/memory", "device_type")
        .unwrap();

    assert!(
        core::str::from_utf8(mem_devt)
            .unwrap()
            .trim_matches(char::from(0))
            == "memory"
    );

    let mem_reg = dtb::get_dtb().get_property("/memory", "reg").unwrap();
    let (start, size) = mem_reg.split_at(core::mem::size_of::<u64>());
    let ram_start = u64::from_be_bytes(start.try_into().unwrap());
    let ram_size = usize::from_be_bytes(size.try_into().unwrap());

    (ram_start, MemorySize(ram_size))
}

pub fn print_ram_info() {
    let (ram_start, ram_size) = get_ramrange();
    info!("      Start Address {:#x}", ram_start);
    info!("      Size {}", ram_size);
}

pub fn print_memory_layout() {
    // Memory Layout
    info!(
        "      {: <30}: [{:p} ~ {:p}]",
        "Kernel",
        unsafe { &super::constants::kernel_start },
        unsafe { &super::constants::kernel_end }
    );
    info!(
        "      {: <30}: [{:p} ~ {:p}]",
        ".text",
        unsafe { &super::constants::__text_start },
        unsafe { &super::constants::__text_end },
    );
    info!(
        "      {: <30}: [{:p} - {:p}]",
        ".bss",
        unsafe { &super::constants::__bss_start },
        unsafe { &super::constants::__bss_end_exclusive }
    );
    info!(
        "      {: <30}: [{:p} ~ {:p}]",
        "boot_core_stack_start",
        unsafe { &super::constants::__boot_core_stack_start },
        unsafe { &super::constants::__boot_core_stack_end_exclusive }
    );
}

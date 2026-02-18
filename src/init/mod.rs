extern crate log as log_crate;
use core::sync::atomic::Ordering;

use crate::arch;
use crate::arch::exception::el::get_current_el;
use crate::arch::memory::mmu;
use crate::bsp;
use crate::bsp::memory::symbols;
use crate::console;
use crate::drivers;
use crate::memory;
use log_crate::info;

#[no_mangle]
pub(crate) unsafe extern "C" fn kernel_main() -> ! {
    // Initialize Exceptions
    arch::exception::irq::irq_disable();
    arch::exception::set_exception_handler();

    // Discover DTB: use firmware-provided address (x0) if available,
    // otherwise scan for FDT magic at known locations.
    let firmware_dtb = arch::start::BOOT_DTB_ADDR.load(Ordering::Relaxed);
    let dtb_addr = drivers::devicetree::find_dtb(firmware_dtb, symbols::RAM_START);

    console::log::init();

    let phys_kernel_tables_base_addr = match memory::kernel_mapper::kernel_map_sections() {
        Err(string) => panic!("Error mapping kernel binary: {}", string),
        Ok(addr) => addr,
    };

    if let Err(e) = memory::mmu::init(phys_kernel_tables_base_addr) {
        panic!("Enabling MMU failed: {}", e);
    }

    // NOTE: No printing between MMU enable and UART re-init.
    // After MMU is on, the physical UART address 0x0900_0000 is unmapped.
    // We must init the MMIO allocator and remap the UART first.

    memory::mmu::init_mmio_allocator();
    bsp::init_drivers(false, dtb_addr);

    // UART is now remapped to a virtual address — safe to print again.
    println!("MMU enabled.");

    // Initialize Interrupts
    bsp::init_irq();

    // Initialize Timer Interrupt
    arch::timer::init_irq();

    arch::exception::irq::irq_enable();
    arch::exception::irq::fiq_enable();

    let ver = env!("CARGO_PKG_VERSION");

    println!("     _________  _________ ___  ____  _____");
    println!("    / ___/ __ \\/ ___/ __ `__ \\/ __ \\/ ___/");
    println!("   / /__/ /_/ (__  ) / / / / / /_/ (__  ) ");
    println!("   \\___/\\____/____/_/ /_/ /_/\\____/____/  v{}", ver);
    println!();

    info!("Provided DTB address: {:#x}", firmware_dtb);
    info!("DTB found at {:#x}", dtb_addr);

    println!(
        "kernel space: {:#x} ~ {:#x}",
        symbols::kernel_range().start,
        symbols::kernel_range().end
    );

    println!("********* MMU Status *********");
    mmu::print_stat();
    memory::kernel_mapper::log_mapping();

    info!("Timer Status: ");
    arch::timer::print_timer_status();
    info!(
        "Timer Resolution: {}ns",
        arch::timer::resolution().as_nanos()
    );

    info!("Current Exception Level: {}", get_current_el());

    info!("Exception handling state:");
    arch::exception::print_state();

    info!("Registered IRQ handlers:");
    drivers::gicv3::print_interrupts();

    info!("Echoing Inputs");
    info!("Waiting for interrupts...");

    let console = console::console();
    console.clear_rx();

    loop {
        arch::halt();
    }
}

use crate::arch::drivers::devicetree;
use crate::arch::drivers::pl011::{self, PL011_UART};
use crate::arch::irq;
use crate::console::register_console;

pub mod memory;

pub fn init() {
    // Initialize Device Tree
    devicetree::init(0x40000000);

    // Initialize UART
    let uart_addr = {
        let stdout = devicetree::get_property("/chosen", "stdout-path").unwrap();
        core::str::from_utf8(stdout)
            .unwrap()
            .trim_matches(char::from(0))
            .split_once('@')
            .map(|(_, addr)| u32::from_str_radix(addr, 16).unwrap())
            .unwrap()
    }; // 0x09000000

    pl011::init(uart_addr);
    // pl011::init(0x09000000);
    register_console(PL011_UART.get().unwrap());

    // Initialize GIC
    // Check Compatible GIC
    let compat =
        core::str::from_utf8(devicetree::get_property("/intc", "compatible").unwrap()).unwrap();
    if !compat.contains("arm,gic-v3") {
        panic!("Compatible GIC (arm,gic-v3) Not Found");
    }

    // Parse GICD & GICC from the devicetree /intc reg
    let reg = devicetree::get_property("/intc", "reg").unwrap();

    // GIC Distributor interface (GICD)
    let (slice, residual_slice) = reg.split_at(core::mem::size_of::<u64>());
    let gicd_start = u64::from_be_bytes(slice.try_into().unwrap());
    let (slice, residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let gicd_size = u64::from_be_bytes(slice.try_into().unwrap());

    // GIC Redistributors (GICR), one range per redistributor region
    let (slice, residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let gicr_start = u64::from_be_bytes(slice.try_into().unwrap());
    let (slice, _residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let gicr_size = u64::from_be_bytes(slice.try_into().unwrap());

    let gicd_start: *mut u64 = gicd_start as _; // 0x08000000
    let gicr_start: *mut u64 = gicr_start as _; // 0x080A0000

    // TODO: allocate gicd and gicr to virtualmem
    irq::init_gic(gicd_start, gicr_start).expect("Failed to initialize GIC");
    // irq::init_gic(0x08000000 as _, 0x080A0000 as _).expect("Failed to initialize GIC");
}

pub fn init_irq() {
    // Initialize Interrupts
    pl011::init_irq();
}

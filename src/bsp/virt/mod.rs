use crate::arch::drivers::devicetree;
use crate::arch::drivers::pl011::{self, PL011_UART};
use crate::arch::irq;
use crate::console::register_console;

pub mod memory;

fn init_device_tree() {
    __println!("Initializing DeviceTree");
    devicetree::init(0x4000_0000);
    __println!("DeviceTree Initialization Successful");
}

fn init_uart() {
    // Initialize UART
    __println!("Initializing PL011 UART");
    // let uart_addr_start: usize = {
    //     let stdout = devicetree::get_property("/chosen", "stdout-path").unwrap();
    //     core::str::from_utf8(stdout)
    //         .unwrap()
    //         .trim_matches(char::from(0))
    //         .split_once('@')
    //         .map(|(_, addr)| u32::from_str_radix(addr, 16).unwrap())
    //         .unwrap()
    // } as usize; // 0x09000000

    let uart_addr_start = 0x0900_0000;
    let uart_addr_end: usize = uart_addr_start + 0x1000;

    let virt_addr =
        memory::kernel_map_mmio("PL011 UART", uart_addr_start.into(), uart_addr_end.into());
    __println!("virt_addr: {}", virt_addr);

    pl011::init(virt_addr.into());
    register_console(PL011_UART.get().unwrap());
    __println!("PL011 Initialization Successful");
}

fn init_gicv3() {
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
    let gicd_start = u64::from_be_bytes(slice.try_into().unwrap()) as usize;
    let (slice, residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let gicd_size = u64::from_be_bytes(slice.try_into().unwrap()) as usize;
    let gicd_virt_addr: usize =
        memory::kernel_map_mmio("GICD", gicd_start.into(), (gicd_start + gicd_size).into()).into();

    // GIC Redistributors (GICR), one range per redistributor region
    let (slice, residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let gicr_start = u64::from_be_bytes(slice.try_into().unwrap()) as usize;
    let (slice, _residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let gicr_size = u64::from_be_bytes(slice.try_into().unwrap()) as usize;
    let gicr_virt_addr: usize =
        memory::kernel_map_mmio("GICR", gicr_start.into(), (gicr_start + gicr_size).into()).into();

    let gicd_start: *mut u64 = gicd_virt_addr as _; // 0x08000000
    let gicr_start: *mut u64 = gicr_virt_addr as _; // 0x080A0000

    irq::init_gic(gicd_start, gicr_start).expect("Failed to initialize GIC");
    // irq::init_gic(0x08000000 as _, 0x080A0000 as _).expect("Failed to initialize GIC");
}

pub fn init_drivers() {
    init_device_tree();
    init_uart();
    init_gicv3();
}

pub fn init_irq() {
    // Initialize Interrupts
    pl011::init_irq();
}

use crate::arch::drivers::devicetree;
use crate::arch::drivers::pl011::{self, PL011_UART};
use crate::arch::irq;
use crate::bsp::memory::symbols::DEVICE_TREE_START;
use crate::console::register_console;

pub mod memory;

fn init_device_tree() {
    devicetree::init(DEVICE_TREE_START);
}

fn init_uart(real: bool, baud_rate: u32) {
    fn get_uart_freq() -> u32 {
        u32::from_be_bytes(
            devicetree::get_property("/apb-pclk", "clock-frequency")
                .expect("Property not found")
                .try_into()
                .expect("Property is not 4 bytes long"),
        )
    }

    // Get UART base address and size from device tree
    let reg = devicetree::get_property("/pl011", "reg").unwrap();
    let (uart_start, rest) = devicetree::dt_read_u64(&reg); // Typically 0x0900_0000 for PL011 UART
    let (uart_size, _) = devicetree::dt_read_u64(rest); // Typically 0x1000 for PL011 UART

    let uart_freq = get_uart_freq();

    if real {
        pl011::init(uart_start, uart_freq, baud_rate);
    } else {
        let virt_addr = memory::kernel_map_mmio(
            "PL011 UART",
            uart_start.into(),
            (uart_start + uart_size).into(),
        );
        pl011::init(virt_addr.into(), uart_freq, baud_rate);
    };

    register_console(PL011_UART.get().unwrap());
}

fn init_gicv3(real: bool) {
    let compat =
        core::str::from_utf8(devicetree::get_property("/intc", "compatible").unwrap()).unwrap();
    if !compat.contains("arm,gic-v3") {
        panic!("Compatible GIC (arm,gic-v3) Not Found");
    }

    let reg = devicetree::get_property("/intc", "reg").unwrap();

    // GIC Distributor interface (GICD)
    let (gicd_start, rest) = devicetree::dt_read_u64(&reg);
    let (gicd_size, rest) = devicetree::dt_read_u64(rest);
    let gicd_virt_addr: usize =
        memory::kernel_map_mmio("GICD", gicd_start.into(), (gicd_start + gicd_size).into()).into();

    // GIC Redistributors (GICR), one range per redistributor region
    let (gicr_start, rest) = devicetree::dt_read_u64(rest);
    let (gicr_size, _) = devicetree::dt_read_u64(rest);
    let gicr_virt_addr: usize =
        memory::kernel_map_mmio("GICR", gicr_start.into(), (gicr_start + gicr_size).into()).into();

    if real {
        irq::init_gic(gicd_start as _, gicr_start as _).expect("Failed to initialize GIC");
    } else {
        irq::init_gic(gicd_virt_addr as *mut u64, gicr_virt_addr as *mut u64)
            .expect("Failed to initialize GIC");
    }
}

pub fn init_drivers(real: bool) {
    init_device_tree();
    init_uart(real, 115200);
    init_gicv3(real);
}

pub fn init_irq() {
    // Initialize Interrupts
    pl011::init_irq();
}

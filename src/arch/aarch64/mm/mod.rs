pub fn print_mmap() {
//     println!("
//     | VIRT_FLASH                                                       | 0x00000000 | 0x08000000   | 0x08000000   |
//     | VIRT_CPUPERIPHS                                                  | 0x08000000 | 0x08020000   | 0x00020000   |
//     | VIRT_GIC_DIST                                                    | 0x08000000 | 0x08010000   | 0x00010000   |
//     | VIRT_GIC_CPU                                                     | 0x08010000 | 0x08020000   | 0x00010000   |
//     | VIRT_GIC_V2M                                                     | 0x08020000 | 0x08021000   | 0x00001000   |
//     | VIRT_GIC_HYP                                                     | 0x08030000 | 0x08040000   | 0x00010000   |
//     | VIRT_GIC_VCPU                                                    | 0x08040000 | 0x08050000   | 0x00010000   |
//     | VIRT_GIC_ITS                                                     | 0x08080000 | 0x080A0000   | 0x00020000   |
//     | VIRT_GIC_REDIST                                                  | 0x080A0000 | 0x09000000   | 0x00F60000   |
//     | VIRT_UART                                                        | 0x09000000 | 0x09001000   | 0x00001000   |
//     | VIRT_RTC                                                         | 0x09010000 | 0x09011000   | 0x00001000   |
//     | VIRT_FW_CFG                                                      | 0x09020000 | 0x09020018   | 0x00000018   |
//     | VIRT_GPIO                                                        | 0x09030000 | 0x09031000   | 0x00001000   |
//     | VIRT_SECURE_UART                                                 | 0x09040000 | 0x09041000   | 0x00001000   |
//     | VIRT_SMMU                                                        | 0x09050000 | 0x09070000   | 0x00020000   |
//     | VIRT_PCDIMM_ACPI                                                 | 0x09070000 | 0x09070018   | 0x00000018   |
//     | VIRT_ACPI_GED                                                    | 0x09080000 | 0x09080004   | 0x00000004   |
//     | VIRT_NVDIMM_ACPI                                                 | 0x09090000 | 0x09090004   | 0x00000004   |
//     | VIRT_PVTIME                                                      | 0x090A0000 | 0x090B0000   | 0x00010000   |
//     | VIRT_SECURE_GPIO                                                 | 0x090B0000 | 0x090B1000   | 0x00001000   |
//     | VIRT_MMIO                                                        | 0x0A000000 | 0x0A000200   | 0x00000200   |
//     | VIRT_PLATFORM_BUS                                                | 0x0C000000 | 0x0E000000   | 0x02000000   |
//     | VIRT_SECURE_MEM                                                  | 0x00000000 | 0x01000000   | 0x01000000   |
//     | VIRT_PCIE_MMIO                                                   | 0x10000000 | 0x3EFF0000   | 0x2eff0000   |
//     | VIRT_PCIE_PIO                                                    | 0x3EFF0000 | 0x3F000000   | 0x00010000   |
//     | VIRT_PCIE_ECAM                                                   | 0x3F000000 | 0x40000000   | 0x01000000   |
//     | VIRT_MEM                                                         | 0x40000000 | 0x4040000000 | 0x4000000000 |
//     | QEMU_DTB                                                         | 0x40000000 | 0x40200000   | 0x00200000   |
//     | COSMOS_RAM_START                                                 | 0x40200000 |              |              |
// ");
    let names: [&str; 26] = [
        "VIRT_FLASH",
        "VIRT_CPUPERIPHS",
        "VIRT_GIC_DIST",
        "VIRT_GIC_CPU",
        "VIRT_GIC_V2M",
        "VIRT_GIC_HYP",
        "VIRT_GIC_VCPU",
        "VIRT_GIC_ITS",
        "VIRT_GIC_REDIST",
        "VIRT_UART",
        "VIRT_RTC",
        "VIRT_FW_CFG",
        "VIRT_GPIO",
        "VIRT_SECURE_UART",
        "VIRT_SMMU",
        "VIRT_PCDIMM_ACPI",
        "VIRT_ACPI_GED",
        "VIRT_NVDIMM_ACPI",
        "VIRT_PVTIME",
        "VIRT_SECURE_GPIO",
        "VIRT_MMIO",
        "VIRT_PLATFORM_BUS",
        "VIRT_SECURE_MEM",
        "VIRT_PCIE_MMIO",
        "VIRT_PCIE_PIO",
        "VIRT_PCIE_ECAM",
    ];

    let addrs: [u64; 26] = [
        0x00000000, 0x08000000, 0x08000000, 0x08010000, 0x08020000, 0x08030000, 0x08040000,
        0x08080000, 0x080A0000, 0x09000000, 0x09010000, 0x09020000, 0x09030000, 0x09040000,
        0x09050000, 0x09070000, 0x09080000, 0x09090000, 0x090A0000, 0x090B0000, 0x0A000000,
        0x0C000000, 0x00000000, 0x10000000, 0x3EFF0000, 0x3F000000,
    ];

    let lens: [u64; 26] = [
        0x08000000, 0x08020000, 0x08010000, 0x08020000, 0x08021000, 0x08040000, 0x08050000,
        0x080A0000, 0x09000000, 0x09001000, 0x09011000, 0x09020018, 0x09031000, 0x09041000,
        0x09070000, 0x09070018, 0x09080004, 0x09090004, 0x090B0000, 0x090B1000, 0x0A000200,
        0x0E000000, 0x01000000, 0x3EFF0000, 0x3F000000, 0x40000000,
    ];

    for i in 0..26 {
        println!("---------------0x{:08X}---------------", addrs[i]);
        println!("| {:<20} 0x{:08X}", names[i], lens[i]);

        // println!("| {:<30} | 0x{:08X} | 0x{:08X}   | 0x{:08X}   |", names[i], addrs[i], addrs[i] + lens[i], lens[i]);
    }
}

pub mod console;
pub mod constants;
pub mod dtb;
pub mod exception;
pub mod mm;
pub mod pl011;
pub mod start;
pub mod test;
pub mod timer;

pub use constants::*;

pub fn get_cpus() -> usize {
    let dtb = &dtb::get_dtb();
    dtb.enum_subnodes("/cpus")
        .filter(|cpu| cpu.split('@').next().unwrap() == "cpu")
        .count()
}

pub fn get_ramrange() -> (u64, u64) {
    let dtb = &dtb::get_dtb();
    let mem_devt = dtb.get_property("/memory", "device_type").unwrap();
    assert!(
        core::str::from_utf8(mem_devt)
            .unwrap()
            .trim_matches(char::from(0))
            == "memory"
    );

    let mem_reg = dtb.get_property("/memory", "reg").unwrap();
    let (start, size) = mem_reg.split_at(core::mem::size_of::<u64>());
    let ram_start = u64::from_be_bytes(start.try_into().unwrap());
    let ram_size = u64::from_be_bytes(size.try_into().unwrap());

    (ram_start, ram_size)
}

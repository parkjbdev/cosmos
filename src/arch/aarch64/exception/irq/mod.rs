use arm_gic::gicv3::GicV3;
use hermit_dtb::Dtb;

pub mod irq;
pub mod irq_type;

pub fn init_gic(dtb: &Dtb) -> GicV3 {
    // Check Compatible GIC
    let compat = core::str::from_utf8(dtb.get_property("/intc", "compatible").unwrap()).unwrap();
    if !compat.contains("arm,gic-v3") {
        panic!("Compatible GIC (arm,gic-v3) Not Found");
    }

    // Parse GICD & GICC from the dtb /intc reg
    let reg = dtb.get_property("/intc", "reg").unwrap();

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

    let gicd_start: *mut u64 = gicd_start as _;
    let gicr_start: *mut u64 = gicr_start as _;

    // TODO: allocate gicd and gicr to virtualmem
    let mut gic = unsafe { GicV3::new(gicd_start, gicr_start) };
    gic.setup();
    GicV3::set_priority_mask(0xff);

    gic
}


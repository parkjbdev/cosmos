use core::arch::asm;

pub fn semihosting_call(num: u64, arg0: u64, arg1: u64) {
    unsafe {
        asm!(
            "hlt #0xF000",
            in("x0") num,
            in("x1") arg0,
            in("x2") arg1,
            options(nostack)
        );
    }
}


pub fn semihosting_exit(status: u64) {
    #[repr(C)]
    struct QEMUParameterBlock {
        arg0: u64,
        arg1: u64,
    }

    let block = &QEMUParameterBlock {
        arg0: 0x20026,
        arg1: 1,
    };

    unsafe {
        asm!(
            "hlt #0xF000",
            in("x0") 0x18,
            in("x1") block as *const _ as u64,
            options(nostack)
        );
    }

    loop {
        unsafe { asm!("wfe", options(nomem, nostack)) };
    }
}

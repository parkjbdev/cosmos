// Scheduler Interrupt
const RESCHED_SGI: u32 = 0;

fn init() {
    let scheduler = Interrupt::new(
        RESCHED_SGI,
        1,
        0x00,
        Some(|state| {
            println!("Scheduler Interrupt");
            true
        }),
        Some("Scheduler"),
    )
    .register();
}


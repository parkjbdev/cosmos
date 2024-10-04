pub trait IRQHandler {
    fn handler(&self, cb: fn());
}

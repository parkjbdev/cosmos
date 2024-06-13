pub trait IRQHandler {
    fn handler(&mut self, cb: fn());
}

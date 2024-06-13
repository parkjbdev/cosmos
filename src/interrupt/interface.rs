pub trait IRQHandler {
    fn handler(&mut self);
}

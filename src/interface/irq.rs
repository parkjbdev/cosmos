pub trait Interrupt {
    fn init(&mut self);
    fn handler(&self);
}

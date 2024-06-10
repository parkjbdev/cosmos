pub trait RegisterInterrupt {
    fn init_irq(&self);
    fn handler(&mut self);
}

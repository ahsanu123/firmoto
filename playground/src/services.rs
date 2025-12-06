mod spi_service;

// TODO: use better implementation for deinit like Drop trait
pub trait ServiceTrait {
    fn init(&mut self);
    fn deinit(&mut self);
}

pub mod spi_service;

// TODO: use better implementation for deinit like Drop trait
pub trait ServiceTrait {
    fn init(&mut self);
    fn deinit(&mut self);
}

pub trait ConfigureableServiceTrait {
    type ConfigSetType;
    type ConfigGetType;

    fn set_config(&mut self, config: &Self::ConfigSetType);
    fn get_config(&mut self) -> Self::ConfigGetType;
}

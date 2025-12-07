pub mod concrete_spi_service;
pub mod mock_spi_service;
pub mod spi_service_traits;

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

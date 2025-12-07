use crate::{
    firmoto_schema_generated::firmoto::ValueT,
    service_provider::get_spi_service,
    services::{ServiceTrait, spi_service_traits::SpiServiceTrait},
};
use alloc::vec::Vec;
pub struct SpiController;

enum SpiControllerError {
    SPI,
}

impl SpiController {
    fn write_u8(args: Vec<ValueT>) -> Result<(), SpiControllerError> {
        // let spi_service: &mut dyn ServiceTrait = get_spi_service();
        // spi_service.init();

        // NOTE:
        // parse args then pass
        // parsed args to real service:
        // - spi_service.write_u8(addr, value)
        // return error if vec arg
        // is not providing corrent argument

        // NOTE: should return Vec<Retval>
        Ok(())
    }
    // etc etc
}

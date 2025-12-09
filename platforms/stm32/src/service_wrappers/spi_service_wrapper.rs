use crate::service_provider::spi_service::STM32_SPI_SERVICE;
use firmoto::{
    service_wrappers::SpiServiceWrapperTrait,
    services::spi_service_traits::{SpiServiceError, SpiServiceTrait as _},
};

pub struct SpiServiceWrapper;

impl SpiServiceWrapperTrait for SpiServiceWrapper {
    fn write_u8(&mut self, address: u8, data: u8) -> Result<(), SpiServiceError> {
        STM32_SPI_SERVICE.lock(|cell| {
            if let Some(spi_dev) = cell.borrow_mut().as_mut() {
                let _ = spi_dev.write_u8(address, data);
            }
        });
        Ok(())
    }

    fn read_u16(&mut self, address: u8) -> Result<u16, SpiServiceError> {
        let mut result: u16 = 0u16;
        STM32_SPI_SERVICE.lock(|cell| {
            if let Some(spi_dev) = cell.borrow_mut().as_mut() {
                result = spi_dev.read_u16(address);
            }
        });
        Ok(result)
    }

    fn read_u8(&mut self, address: u8) -> Result<u8, SpiServiceError> {
        let mut result: u8 = 0u8;
        STM32_SPI_SERVICE.lock(|cell| {
            if let Some(spi_dev) = cell.borrow_mut().as_mut() {
                result = spi_dev.read_u8(address);
            }
        });
        Ok(result)
    }
}

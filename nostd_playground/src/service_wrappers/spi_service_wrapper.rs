use crate::{
    service_provider::SPI_SERVICE,
    services::spi_service_traits::{SpiServiceError, SpiServiceTrait as _},
};

pub trait SpiServiceWrapperTrait {
    fn write_u8(address: u8, data: u8) -> Result<(), SpiServiceError>;

    fn read_u16(address: u8) -> Result<u16, SpiServiceError>;
    fn read_u8(address: u8) -> Result<u8, SpiServiceError>;
}

pub struct SpiServiceWrapper;

impl SpiServiceWrapperTrait for SpiServiceWrapper {
    fn write_u8(address: u8, data: u8) -> Result<(), SpiServiceError> {
        SPI_SERVICE.lock(|cell| {
            if let Some(spi_dev) = cell.borrow_mut().as_mut() {
                let _ = spi_dev.write_u8(address, data);
            }
        });
        Ok(())
    }

    fn read_u16(address: u8) -> Result<u16, SpiServiceError> {
        let mut result: u16 = 0u16;
        SPI_SERVICE.lock(|cell| {
            if let Some(spi_dev) = cell.borrow_mut().as_mut() {
                result = spi_dev.read_u16(address);
            }
        });
        Ok(result)
    }

    fn read_u8(address: u8) -> Result<u8, SpiServiceError> {
        let mut result: u8 = 0u8;
        SPI_SERVICE.lock(|cell| {
            if let Some(spi_dev) = cell.borrow_mut().as_mut() {
                result = spi_dev.read_u8(address);
            }
        });
        Ok(result)
    }
}

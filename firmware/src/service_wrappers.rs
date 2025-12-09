use crate::services::spi_service_traits::SpiServiceError;

pub trait SpiServiceWrapperTrait {
    fn write_u8(&mut self, address: u8, data: u8) -> Result<(), SpiServiceError>;
    fn read_u16(&mut self, address: u8) -> Result<u16, SpiServiceError>;
    fn read_u8(&mut self, address: u8) -> Result<u8, SpiServiceError>;
}

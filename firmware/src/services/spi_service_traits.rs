use crate::services::service_traits::ServiceTrait;

// TODO:
pub enum SpiServiceError {}

pub trait SpiServiceTrait: ServiceTrait {
    fn write_u8(&mut self, address: u8, data: u8) -> Result<(), SpiServiceError>;
    fn read_u16(&mut self, address: u8) -> u16;
    fn read_u8(&mut self, address: u8) -> u8;
    fn read_n<const N: usize>(&mut self, address: u8, buffer: &mut [u8; N]);
}

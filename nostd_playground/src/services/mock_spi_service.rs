use cortex_m_semihosting::hprintln;

use crate::services::{
    ServiceTrait,
    spi_service_traits::{SpiServiceError, SpiServiceTrait},
};

pub struct MockSpiService;

impl ServiceTrait for MockSpiService {
    fn init(&mut self) {
        hprintln!("init");
    }

    fn deinit(&mut self) {
        hprintln!("deinit");
    }
}

impl SpiServiceTrait for MockSpiService {
    fn write_u8(&mut self, address: u8, data: u8) -> Result<(), SpiServiceError> {
        hprintln!("write_u8 {} => {}", address, data);
        Ok(())
    }

    fn read_u16(&mut self, address: u8) -> u16 {
        hprintln!("write_u8 {}", address);
        0u16
    }

    fn read_u8(&mut self, address: u8) -> u8 {
        hprintln!("write_u8 {}", address);
        0u8
    }

    fn read_n<const N: usize>(&mut self, address: u8, _buffer: &mut [u8; N]) {
        hprintln!("write_u8 {}", address);
    }
}

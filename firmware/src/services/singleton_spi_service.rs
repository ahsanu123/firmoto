use embassy_time::Delay;

use crate::services::{ServiceTrait, spi_service_traits::SpiServiceTrait};

struct SingletonSpiService {
    pub delay: Delay,
}

impl ServiceTrait for SingletonSpiService {
    fn init(&mut self) {
        todo!()
    }

    fn deinit(&mut self) {
        todo!()
    }
}

impl SpiServiceTrait for SingletonSpiService {
    fn write_u8(
        &mut self,
        address: u8,
        data: u8,
    ) -> Result<(), super::spi_service_traits::SpiServiceError> {
        todo!()
    }

    fn read_u16(&mut self, address: u8) -> u16 {
        todo!()
    }

    fn read_u8(&mut self, address: u8) -> u8 {
        todo!()
    }

    fn read_n<const N: usize>(&mut self, address: u8, buffer: &mut [u8; N]) {
        todo!()
    }
}

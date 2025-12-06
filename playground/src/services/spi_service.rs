use anyhow::{Ok, Result};
use embedded_hal::{
    delay::DelayNs,
    spi::{Operation, SpiDevice},
};

use crate::services::ServiceTrait;

pub trait SpiServiceTrait: ServiceTrait {
    fn write_u8(&mut self, address: u8, data: u8) -> Result<()>;

    fn read_u16(&mut self, address: u8) -> u16;
    fn read_u8(&mut self, address: u8) -> u8;
    fn read_n<const N: usize>(&mut self, address: u8, buffer: &mut [u8; N]);
}

pub struct SpiService<SPI, Delay>
where
    SPI: SpiDevice,
    Delay: DelayNs,
{
    spi: SPI,
    delay: Delay,
}

impl<SPI, Delay> SpiService<SPI, Delay>
where
    SPI: SpiDevice,
    Delay: DelayNs,
{
    pub fn new(spi: SPI, delay: Delay) -> Self {
        let mut instance = Self { spi, delay };
        instance.init();

        instance
    }
}

impl<SPI, Delay> ServiceTrait for SpiService<SPI, Delay>
where
    SPI: SpiDevice,
    Delay: DelayNs,
{
    fn init(&mut self) {
        todo!()
    }

    fn deinit(&mut self) {
        todo!()
    }
}

impl<SPI, Delay> SpiServiceTrait for SpiService<SPI, Delay>
where
    SPI: SpiDevice,
    Delay: DelayNs,
{
    fn write_u8(&mut self, address: u8, data: u8) -> Result<()> {
        let data = data | 0x80;
        let _ = self.spi.write(&[address, data]);
        Ok(())
    }

    fn read_u8(&mut self, address: u8) -> u8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read_n::<1>(address, &mut buffer);

        buffer[0]
    }

    fn read_u16(&mut self, address: u8) -> u16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read_n::<2>(address, &mut buffer);

        u16::from_be_bytes(buffer)
    }

    fn read_n<const N: usize>(&mut self, address: u8, buffer: &mut [u8; N]) {
        let address = address & 0x7F;

        let _ = self
            .spi
            .transaction(&mut [Operation::Write(&[address]), Operation::Read(buffer)]);
    }
}

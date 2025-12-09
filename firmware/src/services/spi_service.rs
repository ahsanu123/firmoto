use crate::services::{
    service_traits::ServiceTrait,
    spi_service_traits::{SpiServiceError, SpiServiceTrait},
};
use embassy_embedded_hal::{SetConfig, shared_bus::blocking::spi::SpiDeviceWithConfig};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embedded_hal::{
    delay::DelayNs,
    digital::OutputPin,
    spi::{Operation, SpiBus, SpiDevice},
};

pub struct ConcreteSpiService<'a, M, BUS, CS, Delay>
where
    M: RawMutex,
    BUS: SpiBus + SetConfig,
    CS: OutputPin,
    Delay: DelayNs,
{
    pub spi: SpiDeviceWithConfig<'a, M, BUS, CS>,
    pub delay: Delay,
    pub config: <BUS as SetConfig>::Config,
}

impl<'a, M, BUS, CS, Delay> ConcreteSpiService<'a, M, BUS, CS, Delay>
where
    M: RawMutex,
    BUS: SpiBus + SetConfig,
    CS: OutputPin,
    Delay: DelayNs,
{
    pub fn new(
        spi: SpiDeviceWithConfig<'a, M, BUS, CS>,
        delay: Delay,
        config: <BUS as SetConfig>::Config,
    ) -> Self {
        let mut instance = Self { spi, delay, config };
        instance.init();

        instance
    }

    pub fn set_config(&mut self, config: <BUS as SetConfig>::Config) {
        self.spi.set_config(config);
    }
}

impl<'a, M, BUS, CS, Delay> ServiceTrait for ConcreteSpiService<'a, M, BUS, CS, Delay>
where
    M: RawMutex,
    BUS: SpiBus + SetConfig,
    CS: OutputPin,
    Delay: DelayNs,
{
    fn init(&mut self) {
        todo!()
    }

    fn deinit(&mut self) {
        todo!()
    }
}

impl<'a, M, BUS, CS, Delay> SpiServiceTrait for ConcreteSpiService<'a, M, BUS, CS, Delay>
where
    M: RawMutex,
    BUS: SpiBus + SetConfig,
    CS: OutputPin,
    Delay: DelayNs,
{
    fn write_u8(&mut self, address: u8, data: u8) -> Result<(), SpiServiceError> {
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

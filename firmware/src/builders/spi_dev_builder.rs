// TODO:
// let p = embassy_stm32::init(Default::default());
//
// let mut spi_config = Config::default();
// spi_config.frequency = Hertz(1_000_000);
//
// let cs = Output::new(p.PA2, Level::High, Speed::VeryHigh);
// let spi = Spi::new_blocking(p.SPI3, p.PC10, p.PC12, p.PC11, spi_config);
// let current_config = spi.get_current_config();
// let spi_bus = NoopMutex::new(RefCell::new(spi));
// let spi_bus = SPI_BUS.init(spi_bus);

// pub struct SpiService<'a, M, BUS, CS, Delay>
// where
//     M: RawMutex,
//     BUS: SpiBus + SetConfig,
//     CS: OutputPin,
//     Delay: DelayNs,
// {
//     pub spi: SpiDeviceWithConfig<'a, M, BUS, CS>,
//     pub delay: Delay,
//     pub config: <BUS as SetConfig>::Config,
// }

use embassy_embedded_hal::SetConfig;
use embedded_hal::{digital::OutputPin, spi::SpiBus};

pub struct SpiDevBuilder<OUTPIN, BUS>
where
    OUTPIN: OutputPin,
    BUS: SpiBus + SetConfig,
{
    pub cs: OUTPIN,
    pub config: <BUS as SetConfig>::Config,
}

impl<OUTPIN, BUS> SpiDevBuilder<OUTPIN, BUS>
where
    OUTPIN: OutputPin,
    BUS: SpiBus + SetConfig,
{
    pub fn build() {
        todo!()
    }

    pub fn with_config(mut self, config: <BUS as SetConfig>::Config) -> Self {
        self.config = config;
        self
    }
}

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

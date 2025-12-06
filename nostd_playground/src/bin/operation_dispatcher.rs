#![no_std]
#![no_main]

use embassy_stm32::{
    gpio::{Level, Output, Speed},
    spi::{Config, Spi},
    time::Hertz,
};
use nostd_playground::allocator::init_allocator;

#[cortex_m_rt::entry]
fn main() -> ! {
    init_allocator();
    let p = embassy_stm32::init(Default::default());

    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);

    let mut spi = Spi::new_blocking(p.SPI3, p.PC10, p.PC12, p.PC11, spi_config);

    let mut cs = Output::new(p.PA0, Level::High, Speed::VeryHigh);

    nostd_playground::exit()
}

#![no_std]
#![no_main]

use core::cell::RefCell;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    mode::Blocking,
    spi::{Config, Spi},
    time::Hertz,
};
use embassy_sync::blocking_mutex::NoopMutex;
use embassy_time::Delay;
use nostd_playground::allocator::init_allocator;
use nostd_playground::services::spi_service::SpiService;
use static_cell::StaticCell;

static SPI_BUS: StaticCell<NoopMutex<RefCell<Spi<'_, Blocking>>>> = StaticCell::new();

// NOTE:
// reference
//  - https://docs.embassy.dev/embassy-embedded-hal/git/default/shared_bus/blocking/spi/index.html
//  - https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/spi.rs
//

#[cortex_m_rt::entry]
fn main() -> ! {
    init_allocator();

    let p = embassy_stm32::init(Default::default());

    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);

    let cs = Output::new(p.PA2, Level::High, Speed::VeryHigh);
    let spi = Spi::new_blocking(p.SPI3, p.PC10, p.PC12, p.PC11, spi_config);
    let current_config = spi.get_current_config();
    let spi_bus = NoopMutex::new(RefCell::new(spi));
    let spi_bus = SPI_BUS.init(spi_bus);

    let spi_device_with_config = SpiDeviceWithConfig::new(spi_bus, cs, spi_config);
    let spi_service = SpiService::new(spi_device_with_config, Delay, current_config);

    nostd_playground::exit()
}

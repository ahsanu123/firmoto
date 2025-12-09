#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m_semihosting::hprintln;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    mode::Blocking,
    spi::{Config, Spi},
    time::Hertz,
};
use embassy_sync::blocking_mutex::{Mutex, raw::ThreadModeRawMutex};
use embedded_hal::spi::SpiDevice as _;
use nostd_playground::allocator::init_allocator;
use static_cell::StaticCell;

type SpiDevWithConfigType =
    SpiDeviceWithConfig<'static, ThreadModeRawMutex, Spi<'static, Blocking>, Output<'static>>;

type StaticBlockingSpiBusType = Mutex<ThreadModeRawMutex, RefCell<Spi<'static, Blocking>>>;

type MutexSpiDevWithConfigType = Mutex<ThreadModeRawMutex, RefCell<Option<SpiDevWithConfigType>>>;

static SPI_DEVICE_WITH_CONFIG: MutexSpiDevWithConfigType = Mutex::new(RefCell::new(None));

static SPI_BUS: StaticCell<StaticBlockingSpiBusType> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    init_allocator();

    hprintln!("will sync work?");
    let p = embassy_stm32::init(Default::default());

    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);

    let cs = Output::new(p.PA2, Level::High, Speed::VeryHigh);
    let spi = Spi::new_blocking(p.SPI3, p.PC10, p.PC12, p.PC11, spi_config);
    let static_bus = SPI_BUS.init(Mutex::new(RefCell::new(spi)));

    SPI_DEVICE_WITH_CONFIG.lock(|spi_dev_with_config| {
        *spi_dev_with_config.borrow_mut() =
            Some(SpiDeviceWithConfig::new(static_bus, cs, spi_config));
    });

    SPI_DEVICE_WITH_CONFIG.lock(|spi_dev_with_config| {
        if let Some(spi_dev) = spi_dev_with_config.borrow_mut().as_mut() {
            let _ = spi_dev.read(&mut [0u8]);
        }
    });

    nostd_playground::exit()
}

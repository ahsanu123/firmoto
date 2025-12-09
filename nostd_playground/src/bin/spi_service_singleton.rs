#![no_std]
#![no_main]

use core::cell::RefCell;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    mode::Blocking,
    spi::{Config, Spi},
    time::Hertz,
};
use embassy_sync::blocking_mutex::{Mutex, raw::ThreadModeRawMutex};
use embassy_time::Delay;
use nostd_playground::{
    allocator::init_allocator,
    services::{spi_service::ConcreteSpiService, spi_service_traits::SpiServiceTrait},
};
use static_cell::StaticCell;

type StaticBlockingSpiBusType = Mutex<ThreadModeRawMutex, RefCell<Spi<'static, Blocking>>>;

type ConcreteSpiServiceType =
    ConcreteSpiService<'static, ThreadModeRawMutex, Spi<'static, Blocking>, Output<'static>, Delay>;

type MutexSpiService = Mutex<ThreadModeRawMutex, RefCell<Option<ConcreteSpiServiceType>>>;

static SPI_SERVICE: MutexSpiService = Mutex::new(RefCell::new(None));

static SPI_BUS: StaticCell<StaticBlockingSpiBusType> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    init_allocator();

    let p = embassy_stm32::init(Default::default());

    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);

    let cs = Output::new(p.PA2, Level::High, Speed::VeryHigh);
    let spi = Spi::new_blocking(p.SPI3, p.PC10, p.PC12, p.PC11, spi_config);
    let current_config = spi.get_current_config();
    let static_bus = SPI_BUS.init(Mutex::new(RefCell::new(spi)));

    let spi_device_with_config = SpiDeviceWithConfig::new(static_bus, cs, spi_config);
    let service = ConcreteSpiService::new(spi_device_with_config, Delay, current_config);

    SPI_SERVICE.lock(|cell| {
        *cell.borrow_mut() = Some(service);
    });

    SPI_SERVICE.lock(|cell| {
        if let Some(spi_dev) = cell.borrow_mut().as_mut() {
            let _ = spi_dev.read_u8(8u8);
        }
    });

    nostd_playground::exit()
}

#![no_std]
#![no_main]

use core::{cell::RefCell, ops::Deref};
use cortex_m_semihosting::hprintln;
use embassy_executor::Spawner;
use embedded_hal::{delay::DelayNs, spi::SpiDevice as EhSpiDevice};

use embassy_embedded_hal::shared_bus::blocking::spi::{SpiDevice, SpiDeviceWithConfig};
use embassy_stm32::{
    Peripherals,
    gpio::{Level, Output, Speed},
    mode::Blocking,
    spi::{Config, Spi},
    time::Hertz,
};
use embassy_sync::blocking_mutex::{
    Mutex, NoopMutex, ThreadModeMutex,
    raw::{NoopRawMutex, ThreadModeRawMutex},
};
use embassy_time::{Delay, Timer};
use firmoto::allocator::init_allocator;
use firmoto::services::spi_service::ConcreteSpiService;
use static_cell::StaticCell;

type Sdwc =
    SpiDeviceWithConfig<'static, ThreadModeRawMutex, Spi<'static, Blocking>, Output<'static>>;

static SPI_DEVICE_WITH_CONFIG: Mutex<ThreadModeRawMutex, RefCell<Option<Sdwc>>> =
    Mutex::new(RefCell::new(None));
type SpiBusTy<'a> = Mutex<ThreadModeRawMutex, RefCell<Spi<'a, Blocking>>>;
type NoopSpiBusTy<'a> = Mutex<NoopRawMutex, RefCell<Spi<'a, Blocking>>>;
static SPI_BUS: StaticCell<NoopSpiBusTy<'_>> = StaticCell::new();

// NOTE:
// reference
//  - https://docs.embassy.dev/embassy-embedded-hal/git/default/shared_bus/blocking/spi/index.html
//  - https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/spi.rs
//

// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
//     init_allocator();
//
//     hprintln!("will sync work?");
//     let p = embassy_stm32::init(Default::default());
//
//     let mut spi_config = Config::default();
//     spi_config.frequency = Hertz(1_000_000);
//
//     let cs = Output::new(p.PA2, Level::High, Speed::VeryHigh);
//     let spi = Spi::new_blocking(p.SPI3, p.PC10, p.PC12, p.PC11, spi_config);
//     let static_bus = SPI_BUS_SC.init(Mutex::new(RefCell::new(spi)));
//
//     SPI_DEVICE_WITH_CONFIG.lock(|spi_dev_with_config| {
//         *spi_dev_with_config.borrow_mut() =
//             Some(SpiDeviceWithConfig::new(static_bus, cs, spi_config));
//     });
//     firmoto::exit()
// }

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
    let service = ConcreteSpiService::new(spi_device_with_config, Delay, current_config);

    hprintln!("success to create spi_service!!!");

    firmoto::exit()
}

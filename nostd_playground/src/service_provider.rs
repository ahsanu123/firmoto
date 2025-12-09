use crate::services::ServiceTrait;
use crate::services::spi_service::ConcreteSpiService;
use crate::services::spi_service_traits::SpiServiceTrait;
use alloc::boxed::Box;
use core::cell::RefCell;
use embassy_stm32::Peripherals;
use embassy_stm32::{gpio::Output, mode::Blocking, spi::Spi};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::blocking_mutex::{Mutex, raw::NoopRawMutex};
use embassy_time::Delay;
use embedded_hal::spi::{ErrorKind, ErrorType, Operation, SpiDevice};

pub type Stm32SpiService<'a> =
    ConcreteSpiService<'a, NoopRawMutex, Spi<'a, Blocking>, Output<'a>, Delay>;
// static CONCRETE_SPI_SERVICE: StaticCell<Stm32SpiService<'_>> = StaticCell::new();

struct Stm32ConcreteSpiService;

impl ServiceTrait for Stm32ConcreteSpiService {
    fn init(&mut self) {
        todo!()
    }

    fn deinit(&mut self) {
        todo!()
    }
}

impl SpiServiceTrait for Stm32ConcreteSpiService {
    fn write_u8(
        &mut self,
        address: u8,
        data: u8,
    ) -> Result<(), crate::services::spi_service_traits::SpiServiceError> {
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

type EEHSpiDevice = embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice<
    'static,
    ThreadModeRawMutex,
    Spi<'static, Blocking>,
    Output<'static>,
>;

pub static SPI_DEVICE_ONLY: Mutex<ThreadModeRawMutex, RefCell<Option<EEHSpiDevice>>> =
    Mutex::new(RefCell::new(None));

// static SPI_DEVICE: Mutex<ThreadModeRawMutex, RefCell<Option>> = Mutex::new(RefCell::new(None));

// static GENERIC_SPI_SINGLETON: Mutex<ThreadModeRawMutex, RefCell<Option<dyn ServiceTrait>>> =
//     Mutex::new(RefCell::new(None));

static SPI_SINGLETON: Mutex<ThreadModeRawMutex, RefCell<Option<Stm32ConcreteSpiService>>> =
    Mutex::new(RefCell::new(None));

fn try_mutex_spi() {
    SPI_SINGLETON.lock(|cell| {
        let mut opt = cell.borrow_mut();
        if let Some(dev) = opt.as_mut() {
            let _ = dev.read_u8(8u8);
        }
    })
}

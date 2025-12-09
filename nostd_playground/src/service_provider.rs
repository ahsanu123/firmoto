use core::cell::RefCell;
use embassy_stm32::{gpio::Output, mode::Blocking, spi::Spi};
use embassy_sync::blocking_mutex::{Mutex, raw::ThreadModeRawMutex};
use embassy_time::Delay;

use crate::services::spi_service::ConcreteSpiService;

type StaticBlockingSpiBusType = Mutex<ThreadModeRawMutex, RefCell<Spi<'static, Blocking>>>;

type ConcreteSpiServiceType =
    ConcreteSpiService<'static, ThreadModeRawMutex, Spi<'static, Blocking>, Output<'static>, Delay>;

type MutexSpiService = Mutex<ThreadModeRawMutex, RefCell<Option<ConcreteSpiServiceType>>>;

pub static SPI_SERVICE: MutexSpiService = Mutex::new(RefCell::new(None));

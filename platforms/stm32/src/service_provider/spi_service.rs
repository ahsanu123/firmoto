use core::cell::RefCell;
use embassy_stm32::{gpio::Output, mode::Blocking, spi::Spi};
use embassy_sync::blocking_mutex::{Mutex, raw::ThreadModeRawMutex};
use embassy_time::Delay;

use firmoto::services::spi_service::ConcreteSpiService;

type Stm32SpiServiceType =
    ConcreteSpiService<'static, ThreadModeRawMutex, Spi<'static, Blocking>, Output<'static>, Delay>;

type MutexSpiService = Mutex<ThreadModeRawMutex, RefCell<Option<Stm32SpiServiceType>>>;

pub static STM32_SPI_SERVICE: MutexSpiService = Mutex::new(RefCell::new(None));

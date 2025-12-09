use crate::services::concrete_spi_service::ConcreteSpiService;
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

struct SpiMo;
impl ErrorType for SpiMo {
    type Error = ErrorKind;
}
impl SpiDevice for SpiMo {
    fn transaction(
        &mut self,
        _operations: &mut [embedded_hal::spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

static PERIPHERAL: Mutex<ThreadModeRawMutex, RefCell<Option<Peripherals>>> =
    Mutex::new(RefCell::new(None));

static SPI: Mutex<ThreadModeRawMutex, RefCell<Option<SpiMo>>> =
    Mutex::new(RefCell::new(Some(SpiMo)));

fn try_mutex_spi() {
    SPI.lock(|cell| {
        let mut opt = cell.borrow_mut();
        if let Some(dev) = opt.as_mut() {
            let _ = dev.transaction(&mut [Operation::Read(&mut [1u8])]);
        }
    })
}

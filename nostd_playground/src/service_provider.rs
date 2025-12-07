use crate::services::spi_service::SpiService;
use alloc::string::String;
use embassy_stm32::{gpio::Output, mode::Blocking, spi::Spi};
use embassy_sync::blocking_mutex::{CriticalSectionMutex, NoopMutex, raw::NoopRawMutex};
use embassy_sync::once_lock::OnceLock;
use embassy_time::Delay;
use static_cell::StaticCell;

pub type Stm32SpiService<'a> = SpiService<'a, NoopRawMutex, Spi<'a, Blocking>, Output<'a>, Delay>;

static SPI_SERVICE: StaticCell<Stm32SpiService<'_>> = StaticCell::new();

pub struct Pet {
    pub name: String,
    pub age: u16,
}

static PET_SERVICE: OnceLock<Pet> = OnceLock::new();

pub fn get_singleton() -> &'static Pet {
    PET_SERVICE.get_or_init(|| Pet {
        name: String::from("Josh"),
        age: 2u16,
    })
}

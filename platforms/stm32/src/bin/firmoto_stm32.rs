#![no_main]
#![no_std]

extern crate alloc;

use firmoto::{
    allocator::init_allocator, controllers::spi_controller::SpiController, dispatcher::Dispatcher,
};
use firmoto_stm32::{
    serial_transport::Stm32SerialTransport,
    service_wrappers::spi_service_wrapper::SpiServiceWrapper,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    init_allocator();

    let spi_controller = SpiController::new(SpiServiceWrapper);

    let mut dispatcher = Dispatcher::new(spi_controller, Stm32SerialTransport);

    // TODO:
    // loop {
    dispatcher.run();
    // }

    firmoto_stm32::exit()
}

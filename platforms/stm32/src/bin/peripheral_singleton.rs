#![no_std]
#![no_main]

use core::cell::RefCell;

use embassy_executor::Spawner;
use embassy_stm32::{
    Config, Peripherals,
    gpio::{Input, Level, Output, Pull, Speed},
    i2c::{self, I2c},
};
use embassy_sync::blocking_mutex::{Mutex, raw::ThreadModeRawMutex};
use firmoto::allocator::init_allocator;

static PERIPHERAL: Mutex<ThreadModeRawMutex, RefCell<Option<Peripherals>>> =
    Mutex::new(RefCell::new(None));

fn peripheral_op<F>(fn_with_peri: F)
where
    F: Fn(&mut Peripherals),
{
    PERIPHERAL.lock(|cell| {
        if let Some(peri) = cell.borrow_mut().as_mut() {
            fn_with_peri(peri);
        }
    });
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    init_allocator();

    let peripheral = embassy_stm32::init(Config::default());

    PERIPHERAL.lock(|cell| {
        *cell.borrow_mut() = Some(peripheral);
    });

    peripheral_op(|p| {
        let mut buf: [u8; 3] = [0u8; 3];
        let mut i2c = I2c::new_blocking(
            p.I2C1.reborrow(),
            p.PB6.reborrow(),
            p.PB7.reborrow(),
            i2c::Config::default(),
        );

        let _ = i2c.blocking_read(7u8, &mut buf);

        drop(i2c);

        let input = Input::new(p.PB6.reborrow(), Pull::Up);
        let output = Output::new(p.PB7.reborrow(), Level::Low, Speed::Medium);

        drop(input);
        drop(output);
    });

    firmoto_stm32::exit()
}

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Input, Level, Output, Pull, Speed},
    i2c::{Config, I2c},
};
use firmoto::allocator::init_allocator;
use firmoto_stm32::utils::logger::{Log, log};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    init_allocator();

    log(Log::Info, "init re init peripheral");

    let p = embassy_stm32::init(Default::default());

    let mut pb6 = p.PB6;
    let mut pb7 = p.PB7;

    let mut buf: [u8; 3] = [0u8; 3];
    let mut i2c = I2c::new_blocking(p.I2C1, pb6.reborrow(), pb7.reborrow(), Config::default());
    let _ = i2c.blocking_read(7u8, &mut buf);

    // drop or de-init i2c
    drop(i2c);

    // then reuse pin to other function
    let _input = Input::new(pb7, Pull::Up);
    let _output = Output::new(pb6, Level::Low, Speed::Medium);

    firmoto_stm32::exit()
}

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use nostd_playground::{
    allocator::init_allocator,
    utils::logger::{Log, log},
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    init_allocator();

    log(Log::Info, "test info");
    log(Log::Warning, "test warning");
    log(Log::Error, "test error");

    nostd_playground::exit()
}

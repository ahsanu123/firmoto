#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use nostd_playground as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    hprintln!("Hello, Fucking!! world!");

    nostd_playground::exit()
}

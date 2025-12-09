// ideas on how to create multiple platform
// SPI_SERVICE is same type (implementing same trait)
// so library see samethink

#[cfg(feature = "")]
mod platform {
    pub use crate::platform::esp32::*;
}

#[cfg(stm32)]
mod platform {
    pub use crate::platform::stm32::*;
}

#[cfg(nrf52)]
mod platform {
    pub use crate::platform::nrf52::*;
}

// use platform::{SPI_SERVICE, init_spi};

mod can_controller;
mod gpio_controller;
mod hello_world_controller;
mod i2c_controller;
mod i2s_controller;
mod modbus_controller;
mod spi_controller;
mod uart_controller;

pub mod transport;

pub mod spi_controller_err {
    pub use super::spi_controller::*;
}
pub mod can_controller_err {
    pub use super::can_controller::*;
}
pub mod gpio_controller_err {
    pub use super::gpio_controller::*;
}
pub mod i2c_controller_err {
    pub use super::i2c_controller::*;
}
pub mod i2s_controller_err {
    pub use super::i2s_controller::*;
}
pub mod modbus_controller_err {
    pub use super::modbus_controller::*;
}
pub mod uart_controller_err {
    pub use super::uart_controller::*;
}

pub mod hello_world_controller_err {
    pub use super::hello_world_controller::*;
}

pub mod flatbuffer_request_parse_err;
pub mod get_err_msg_trait;

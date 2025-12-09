mod spi_read_u16_request;
mod spi_read_u8_request;
mod spi_write_then_read_request;
mod spi_write_u8_request;

pub mod spi_controller_request {
    pub use super::spi_read_u8_request::*;
    pub use super::spi_read_u16_request::*;
    pub use super::spi_write_then_read_request::*;
    pub use super::spi_write_u8_request::*;
}

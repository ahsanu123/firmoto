use alloc::string::String;

use crate::errors::uart_controller_err::UartControllerErr;

pub trait UartControllerTrait {
    fn read(&mut self) -> Result<char, UartControllerErr>;
    fn write(&mut self, c: char) -> Result<(), UartControllerErr>;

    fn write_string(&mut self, string: String) -> Result<(), UartControllerErr>;
    fn read_until_char(&mut self, c: char) -> Result<(), UartControllerErr>;
}

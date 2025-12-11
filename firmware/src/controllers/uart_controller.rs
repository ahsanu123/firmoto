use alloc::string::String;

use crate::{
    errors::uart_controller_err::UartControllerErr,
    service_wrappers::uart_service_wrapper::UartServiceWrapperTrait,
};

pub trait UartControllerTrait {
    fn read(&mut self) -> Result<char, UartControllerErr>;
    fn write(&mut self, c: char) -> Result<(), UartControllerErr>;

    fn write_string(&mut self, string: String) -> Result<(), UartControllerErr>;
    fn read_until_char(&mut self, c: char) -> Result<(), UartControllerErr>;
}

pub struct UartController<W>
where
    W: UartServiceWrapperTrait,
{
    wrapper: W,
}

impl<W> UartController<W>
where
    W: UartServiceWrapperTrait,
{
    pub fn new(wrapper: W) -> Self {
        Self { wrapper }
    }
}

impl<W> UartControllerTrait for UartController<W>
where
    W: UartServiceWrapperTrait,
{
    fn read(&mut self) -> Result<char, UartControllerErr> {
        todo!()
    }

    fn write(&mut self, c: char) -> Result<(), UartControllerErr> {
        todo!()
    }

    fn write_string(&mut self, string: String) -> Result<(), UartControllerErr> {
        todo!()
    }

    fn read_until_char(&mut self, c: char) -> Result<(), UartControllerErr> {
        todo!()
    }
}

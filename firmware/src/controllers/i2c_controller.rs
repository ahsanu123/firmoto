use crate::{
    errors::i2c_controller_err::I2cControllerErr,
    service_wrappers::i2c_service_wrapper::I2cServiceWrapperTrait,
};

pub trait I2cControllerTrait {
    fn begin_transmission(&mut self, addr: u8) -> Result<(), I2cControllerErr>;
    fn end_transmission(&mut self) -> Result<(), I2cControllerErr>;

    fn write(&mut self, data: u8) -> Result<(), I2cControllerErr>;
    fn write_with_addr(&mut self, addr: u8, data: u8) -> Result<(), I2cControllerErr>;

    fn read(&mut self) -> Result<(), I2cControllerErr>;
    fn read_from_addr(&mut self, addr: u8) -> Result<(), I2cControllerErr>;
}

pub struct I2cController<W>
where
    W: I2cServiceWrapperTrait,
{
    wrapper: W,
}

impl<W> I2cController<W>
where
    W: I2cServiceWrapperTrait,
{
    pub fn new(wrapper: W) -> Self {
        Self { wrapper }
    }
}

impl<W> I2cServiceWrapperTrait for I2cController<W> where W: I2cServiceWrapperTrait {}

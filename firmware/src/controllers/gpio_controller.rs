use crate::{
    errors::gpio_controller_err::GpioControllerErr,
    service_wrappers::gpio_service_wrapper::GpioServiceWrapperTrait,
};

pub enum GpioPin {
    PA1,
    PA2,
    // etc
}

pub trait GpioControllerTrait {
    fn set_input(&mut self, pin: GpioPin) -> Result<(), GpioControllerErr>;
    fn set_output(&mut self, pin: GpioPin) -> Result<(), GpioControllerErr>;

    fn set_low(&mut self, pin: GpioPin) -> Result<(), GpioControllerErr>;
    fn set_high(&mut self, pin: GpioPin) -> Result<(), GpioControllerErr>;

    fn toggle(&mut self, pin: GpioPin) -> Result<(), GpioControllerErr>;
}

pub struct GpioController<W>
where
    W: GpioServiceWrapperTrait,
{
    wrapper: W,
}

impl<W> GpioController<W>
where
    W: GpioServiceWrapperTrait,
{
    pub fn new(wrapper: W) -> Self {
        Self { wrapper }
    }
}

impl<W> GpioControllerTrait for GpioController<W>
where
    W: GpioServiceWrapperTrait,
{
    fn set_input(&mut self, pin: GpioPin) -> Result<(), GpioControllerErr> {
        todo!()
    }

    fn set_output(&mut self, pin: GpioPin) -> Result<(), GpioControllerErr> {
        todo!()
    }

    fn set_low(&mut self, pin: GpioPin) -> Result<(), GpioControllerErr> {
        todo!()
    }

    fn set_high(&mut self, pin: GpioPin) -> Result<(), GpioControllerErr> {
        todo!()
    }

    fn toggle(&mut self, pin: GpioPin) -> Result<(), GpioControllerErr> {
        todo!()
    }
}

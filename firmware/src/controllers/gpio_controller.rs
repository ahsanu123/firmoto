use crate::errors::gpio_controller_err::GpioControllerErr;

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

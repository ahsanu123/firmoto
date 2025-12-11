use embassy_time::Duration;

use crate::{
    errors::i2s_controller_err::I2sControllerErr,
    service_wrappers::i2s_service_wrapper::I2sServiceWrapperTrait,
};

pub trait I2sControllerTrait {
    fn configure_output(
        &mut self,
        bclk: u8,
        lrck: u8,
        dout: u8,
        sample_rate: u32,
        bit: u8,
    ) -> Result<(), I2sControllerErr>;

    fn configure_input(
        &mut self,
        bclk: u8,
        lrck: u8,
        dout: u8,
        sample_rate: u32,
        bit: u8,
    ) -> Result<(), I2sControllerErr>;

    fn play_tone(
        &mut self,
        sample_rate: u32,
        freq: u16,
        duration_ms: Duration,
    ) -> Result<(), I2sControllerErr>;

    fn play_pcm(&mut self, data: u32, size: usize) -> Result<(), I2sControllerErr>;

    fn record_sample(&mut self, sample_count: usize) -> Result<(), I2sControllerErr>;
}

pub struct I2sController<W>
where
    W: I2sServiceWrapperTrait,
{
    wrapper: W,
}

impl<W> I2sController<W>
where
    W: I2sServiceWrapperTrait,
{
    pub fn new(wrapper: W) -> Self {
        Self { wrapper }
    }
}

impl<W> I2sControllerTrait for I2sController<W>
where
    W: I2sServiceWrapperTrait,
{
    fn configure_output(
        &mut self,
        bclk: u8,
        lrck: u8,
        dout: u8,
        sample_rate: u32,
        bit: u8,
    ) -> Result<(), I2sControllerErr> {
        todo!()
    }

    fn configure_input(
        &mut self,
        bclk: u8,
        lrck: u8,
        dout: u8,
        sample_rate: u32,
        bit: u8,
    ) -> Result<(), I2sControllerErr> {
        todo!()
    }

    fn play_tone(
        &mut self,
        sample_rate: u32,
        freq: u16,
        duration_ms: Duration,
    ) -> Result<(), I2sControllerErr> {
        todo!()
    }

    fn play_pcm(&mut self, data: u32, size: usize) -> Result<(), I2sControllerErr> {
        todo!()
    }

    fn record_sample(&mut self, sample_count: usize) -> Result<(), I2sControllerErr> {
        todo!()
    }
}

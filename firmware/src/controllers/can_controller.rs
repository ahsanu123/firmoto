use crate::{
    errors::can_controller_err::CanControllerErr,
    service_wrappers::can_service_wrapper::CanServiceWrapperTrait,
};

pub trait CanControllerTrait {
    fn send_frame(&mut self, id: u32, data: &[u8]) -> Result<(), CanControllerErr>;
    fn read_frame(&mut self) -> Result<(), CanControllerErr>;
    fn set_bitrate(&mut self, bit_kbps: u32) -> Result<(), CanControllerErr>;
    fn set_filter(&mut self, id: u32) -> Result<(), CanControllerErr>;
    fn set_mask(&mut self, mask: u32) -> Result<(), CanControllerErr>;
}

pub struct CanController<W>
where
    W: CanServiceWrapperTrait,
{
    wrapper: W,
}

impl<W> CanController<W>
where
    W: CanServiceWrapperTrait,
{
    pub fn new(wrapper: W) -> Self {
        Self { wrapper }
    }
}

impl<W> CanControllerTrait for CanController<W>
where
    W: CanServiceWrapperTrait,
{
    fn send_frame(&mut self, id: u32, data: &[u8]) -> Result<(), CanControllerErr> {
        todo!()
    }

    fn read_frame(&mut self) -> Result<(), CanControllerErr> {
        todo!()
    }

    fn set_bitrate(&mut self, bit_kbps: u32) -> Result<(), CanControllerErr> {
        todo!()
    }

    fn set_filter(&mut self, id: u32) -> Result<(), CanControllerErr> {
        todo!()
    }

    fn set_mask(&mut self, mask: u32) -> Result<(), CanControllerErr> {
        todo!()
    }
}

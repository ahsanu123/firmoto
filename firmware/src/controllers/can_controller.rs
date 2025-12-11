use crate::errors::can_controller_err::CanControllerErr;

pub trait CanControllerTrait {
    fn send_frame(&mut self, id: u32, data: &[u8]) -> Result<(), CanControllerErr>;
    fn read_frame(&mut self) -> Result<(), CanControllerErr>;
    fn set_bitrate(&mut self, bit_kbps: u32) -> Result<(), CanControllerErr>;
    fn set_filter(&mut self, id: u32) -> Result<(), CanControllerErr>;
    fn set_mask(&mut self, mask: u32) -> Result<(), CanControllerErr>;
}

// pub struct CanController{
//     wrapper: CanController
// }

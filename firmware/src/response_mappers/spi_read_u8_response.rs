use crate::{controllers::controller_result::CR, errors::spi_controller_err::SpiControllerErr};
use alloc::vec::Vec;
use core::convert::From;

pub struct SpiReadU8Res;

impl From<CR<SpiReadU8Res, SpiControllerErr>> for Vec<u8> {
    fn from(value: CR<SpiReadU8Res, SpiControllerErr>) -> Self {
        todo!()
    }
}

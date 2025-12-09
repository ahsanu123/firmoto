use crate::{controllers::controller_result::CR, errors::spi_controller_err::SpiControllerErr};
use alloc::vec::Vec;
use core::convert::From;

pub struct SpiWriteU8Res;

impl From<CR<SpiWriteU8Res, SpiControllerErr>> for Vec<u8> {
    fn from(value: CR<SpiWriteU8Res, SpiControllerErr>) -> Self {
        todo!()
    }
}

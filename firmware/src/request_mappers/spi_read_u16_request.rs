use crate::schema_generated::firmoto::ValueT;
use alloc::vec::Vec;
use core::convert::From;

pub struct SpiReadU16Req;

impl From<Vec<ValueT>> for SpiReadU16Req {
    fn from(value: Vec<ValueT>) -> Self {
        todo!()
    }
}
// Vec<u8>: From<Result<SpiWriteU8Res, SpiControllerErr>>

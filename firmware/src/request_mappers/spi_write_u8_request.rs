use crate::schema_generated::firmoto::ValueT;
use alloc::vec::Vec;
use core::convert::From;

pub struct SpiWriteU8Req;

impl From<Vec<ValueT>> for SpiWriteU8Req {
    fn from(value: Vec<ValueT>) -> Self {
        todo!()
    }
}

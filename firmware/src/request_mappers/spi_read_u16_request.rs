use crate::schema_generated::firmoto::ValueT;
use alloc::vec::Vec;
use core::convert::From;

pub struct SpiReadU16Req;

impl From<Vec<ValueT>> for SpiReadU16Req {
    fn from(value: Vec<ValueT>) -> Self {
        todo!()
    }
}

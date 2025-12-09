use crate::schema_generated::firmoto::ValueT;
use alloc::vec::Vec;
use core::convert::From;

pub struct SpiReadU8Req;

impl From<Vec<ValueT>> for SpiReadU8Req {
    fn from(value: Vec<ValueT>) -> Self {
        todo!()
    }
}

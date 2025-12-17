use crate::schema_generated::firmoto::ValueT;
use alloc::vec::Vec;
use core::convert::From;

pub struct SpiReadU8Req {
    pub address: u8,
    pub length: u32,
}

impl From<Vec<ValueT>> for SpiReadU8Req {
    fn from(value: Vec<ValueT>) -> Self {
        todo!()
    }
}

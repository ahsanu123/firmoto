use crate::schema_generated::firmoto::ValueT;
use alloc::vec::Vec;
use core::convert::From;

pub struct SpiWriteThenReadReq;

impl From<Vec<ValueT>> for SpiWriteThenReadReq {
    fn from(value: Vec<ValueT>) -> Self {
        todo!()
    }
}

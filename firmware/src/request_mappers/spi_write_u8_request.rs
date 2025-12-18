use crate::{
    errors::flatbuffer_request_parse_err::FBRequestParseErr, schema_generated::firmoto::ValueT,
};
use alloc::{string::String, vec::Vec};
use core::convert::From;
use firmoto_macro::FromVecTToConcrete;

#[derive(FromVecTToConcrete, Default)]
pub struct SpiWriteU8Req {
    pub address: u8,
    pub length: u32,
}

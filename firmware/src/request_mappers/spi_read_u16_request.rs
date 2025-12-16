use crate::{
    errors::flatbuffer_request_parse_err::FBRequestParseErr,
    schema_generated::firmoto::{FieldType, FieldU8T, ValueT},
};
use alloc::{string::String, vec::Vec};
use core::convert::From;

#[derive(Default)]
pub struct SpiReadU16Req {
    pub address: u8,
    pub length: u32,
}

impl SpiReadU16Req {
    pub fn from(value: Vec<ValueT>) -> Result<Self, FBRequestParseErr>
    where
        Self: Default,
    {
        let mut request = Self::default();

        // NOTE: convert this to macro derive
        // =========================================================
        let address = value
            .iter()
            .find(|pr| pr.name == Some(String::from("address")))
            .ok_or(FBRequestParseErr::FieldNotFound)?;

        let address_val = address
            .value
            .as_field_u8()
            .ok_or(FBRequestParseErr::ValueNotFound)?;

        request.address = address_val.field;

        // =========================================================

        let length = value
            .iter()
            .find(|pr| pr.name == Some(String::from("length")))
            .ok_or(FBRequestParseErr::FieldNotFound)?;

        let length = length
            .value
            .as_field_u32()
            .ok_or(FBRequestParseErr::ValueNotFound)?;

        request.length = length.field;

        // =========================================================

        Ok(request)
    }
}

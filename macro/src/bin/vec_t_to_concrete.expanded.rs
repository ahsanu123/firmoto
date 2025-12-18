#![feature(prelude_import)]
#![no_main]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::{string::String, vec::Vec};
use core::convert::From;
use firmoto::{
    errors::flatbuffer_request_parse_err::FBRequestParseErr,
    schema_generated::firmoto::ValueT,
};
use firmoto_macro::FromVecTToConcrete;
struct Argument {
    name: String,
    eat_before: bool,
    age: u8,
    value: u32,
}
impl Argument {
    pub fn from_arg(vec_value_t: Vec<ValueT>) -> Result<Self, FBRequestParseErr>
    where
        Self: Default,
    {
        let mut request = Self::default();
        let name = vec_value_t
            .iter()
            .find(|pr| pr.name == Some(String::from("name")))
            .ok_or(FBRequestParseErr::FieldNotFound(String::from("name")))?;
        let name_val = name
            .value
            .as_field_string()
            .ok_or(FBRequestParseErr::ValueNotFound(String::from("name")))?;
        request.name = name_val
            .field
            .clone()
            .ok_or(FBRequestParseErr::StringUnParseable(String::from("name")))?;
        let eat_before = vec_value_t
            .iter()
            .find(|pr| pr.name == Some(String::from("eat_before")))
            .ok_or(FBRequestParseErr::FieldNotFound(String::from("eat_before")))?;
        let eat_before_val = eat_before
            .value
            .as_field_bool()
            .ok_or(FBRequestParseErr::ValueNotFound(String::from("eat_before")))?;
        request.eat_before = eat_before_val.field;
        let age = vec_value_t
            .iter()
            .find(|pr| pr.name == Some(String::from("age")))
            .ok_or(FBRequestParseErr::FieldNotFound(String::from("age")))?;
        let age_val = age
            .value
            .as_field_u8()
            .ok_or(FBRequestParseErr::ValueNotFound(String::from("age")))?;
        request.age = age_val.field;
        let value = vec_value_t
            .iter()
            .find(|pr| pr.name == Some(String::from("value")))
            .ok_or(FBRequestParseErr::FieldNotFound(String::from("value")))?;
        let value_val = value
            .value
            .as_field_u32()
            .ok_or(FBRequestParseErr::ValueNotFound(String::from("value")))?;
        request.value = value_val.field;
        Ok(request)
    }
}
#[automatically_derived]
impl ::core::default::Default for Argument {
    #[inline]
    fn default() -> Argument {
        Argument {
            name: ::core::default::Default::default(),
            eat_before: ::core::default::Default::default(),
            age: ::core::default::Default::default(),
            value: ::core::default::Default::default(),
        }
    }
}
fn main() {}

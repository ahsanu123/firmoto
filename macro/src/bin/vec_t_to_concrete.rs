#![no_main]
#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::convert::From;
use firmoto::{
    errors::flatbuffer_request_parse_err::FBRequestParseErr, schema_generated::firmoto::ValueT,
};
use firmoto_macro::FromVecTToConcrete;

#[derive(FromVecTToConcrete, Default)]
struct Argument {
    name: String,
    eat_before: bool,
    age: u8,
    value: u32,
}

fn main() {}

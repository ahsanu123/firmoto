use alloc::{string::String, vec::Vec};

#[derive(Debug)]
pub enum FBRequestParseErr {
    FieldNotFound(String),
    ValueNotFound(String),
    StringUnParseable(String),
}

// TODO:
impl From<FBRequestParseErr> for Vec<u8> {
    fn from(value: FBRequestParseErr) -> Self {
        todo!()
    }
}

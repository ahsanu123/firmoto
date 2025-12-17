use alloc::string::String;

pub enum FBRequestParseErr {
    FieldNotFound(String),
    ValueNotFound(String),
    StringUnParseable(String),
}

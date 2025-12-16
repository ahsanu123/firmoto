#[macro_use]
extern crate firmoto_macro;
use firmoto_macro::FromVecTToConcrete;
struct Argument {
    name: String,
    age: u8,
    value: u32,
}

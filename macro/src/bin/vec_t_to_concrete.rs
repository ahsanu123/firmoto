use firmoto_macro::FromVecTToConcrete;

#[derive(FromVecTToConcrete)]
struct Argument {
    name: String,
    age: u8,
    value: u32,
}

fn main() {
    println!("hello world");
}

#![feature(array_zip, assert_matches, map_try_insert)]

mod assembly;
mod computer;
mod gates;
mod general;
mod infrastructure;
mod instruction;
mod parser;
mod primitive;
mod sequential;

fn main() {
    let a = primitive::Bit::Negative;
    let b = primitive::Bit::Positive;
    let _ = a.nand(b);
    println!("Hello, world!");
}

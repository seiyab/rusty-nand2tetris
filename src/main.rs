#![feature(array_zip)]

mod gates;
mod infrastructure;
mod primitive;
mod sequential;

fn main() {
    let a = primitive::Bit::Negative;
    let b = primitive::Bit::Positive;
    let _ = a.nand(b);
    println!("Hello, world!");
}

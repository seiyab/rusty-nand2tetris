mod gates;
mod infrastructure;
mod primitive;

fn main() {
    let a = primitive::Bit::Negative;
    let b = primitive::Bit::Positive;
    let _ = a.nand(b);
    println!("Hello, world!");
}

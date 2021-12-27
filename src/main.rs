mod primitive;

fn main() {
    let a = primitive::Pin::Negative;
    let b = primitive::Pin::Positive;
    let _ = a.nand(b);
    println!("Hello, world!");
}

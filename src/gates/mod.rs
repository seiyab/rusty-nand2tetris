use super::primitive::Pin;

pub fn not(x: Pin) -> Pin {
    x.nand(Pin::Positive)
}

mod test {
    use super::super::primitive::Pin;
    use super::*;
    #[test]
    fn not_works() {
        assert!(matches!(not(Pin::Positive), Pin::Negative));
        assert!(matches!(not(Pin::Negative), Pin::Positive));
    }
}

use super::primitive::Pin;

pub fn not(x: Pin) -> Pin {
    x.nand(Pin::Positive)
}

pub fn and(x: Pin, y: Pin) -> Pin {
    not(x.nand(y))
}

mod test {
    use super::super::primitive::Pin;
    use super::*;

    #[test]
    fn not_works() {
        assert!(matches!(not(Pin::Positive), Pin::Negative));
        assert!(matches!(not(Pin::Negative), Pin::Positive));
    }

    #[test]
    fn and_works() {
        assert!(matches!(and(Pin::Positive, Pin::Positive), Pin::Positive));
        assert!(matches!(and(Pin::Positive, Pin::Negative), Pin::Negative));
        assert!(matches!(and(Pin::Negative, Pin::Positive), Pin::Negative));
        assert!(matches!(and(Pin::Negative, Pin::Negative), Pin::Negative));
    }
}

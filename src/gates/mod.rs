use super::primitive::Pin;

pub fn not(x: Pin) -> Pin {
    x.nand(Pin::Positive)
}

pub fn and(x: Pin, y: Pin) -> Pin {
    not(x.nand(y))
}

pub fn or(x: Pin, y: Pin) -> Pin {
    not(x).nand(not(y))
}

pub fn xor(x: Pin, y: Pin) -> Pin {
    and(or(x, y), x.nand(y))
}

mod test {
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

    #[test]
    fn or_works() {
        assert!(matches!(or(Pin::Positive, Pin::Positive), Pin::Positive));
        assert!(matches!(or(Pin::Positive, Pin::Negative), Pin::Positive));
        assert!(matches!(or(Pin::Negative, Pin::Positive), Pin::Positive));
        assert!(matches!(or(Pin::Negative, Pin::Negative), Pin::Negative));
    }

    #[test]
    fn xor_works() {
        assert!(matches!(xor(Pin::Positive, Pin::Positive), Pin::Negative));
        assert!(matches!(xor(Pin::Positive, Pin::Negative), Pin::Positive));
        assert!(matches!(xor(Pin::Negative, Pin::Positive), Pin::Positive));
        assert!(matches!(xor(Pin::Negative, Pin::Negative), Pin::Negative));
    }
}

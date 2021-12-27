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

pub fn mux(x: Pin, y: Pin, sel: Pin) -> Pin {
    or(and(sel, x), and(not(sel), y))
}

pub fn dmux(x: Pin, sel: Pin) -> (Pin, Pin) {
    (and(x, sel), and(x, not(sel)))
}

#[cfg(test)]
mod test {
    use super::super::assert_pin_equals;
    use super::*;

    #[test]
    fn not_works() {
        assert!(matches!(not(Pin::Positive), Pin::Negative));
        assert!(matches!(not(Pin::Negative), Pin::Positive));
    }

    #[test]
    fn not_not_is_id() {
        for x in [Pin::Positive, Pin::Negative] {
            assert_pin_equals!(not(not(x)), x);
        }
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

    #[test]
    fn mux_work() {
        for b in [Pin::Positive, Pin::Negative] {
            assert!(matches!(
                mux(Pin::Positive, b, Pin::Positive),
                Pin::Positive,
            ));
            assert!(matches!(
                mux(Pin::Negative, b, Pin::Positive),
                Pin::Negative,
            ));
        }
        for a in [Pin::Positive, Pin::Negative] {
            assert!(matches!(
                mux(a, Pin::Positive, Pin::Negative),
                Pin::Positive,
            ));
            assert!(matches!(
                mux(a, Pin::Negative, Pin::Negative),
                Pin::Negative,
            ));
        }
    }

    #[test]
    fn dmux_work() {
        assert!(matches!(
            dmux(Pin::Positive, Pin::Positive),
            (Pin::Positive, Pin::Negative),
        ));
        assert!(matches!(
            dmux(Pin::Negative, Pin::Positive),
            (Pin::Negative, Pin::Negative),
        ));
        assert!(matches!(
            dmux(Pin::Positive, Pin::Negative),
            (Pin::Negative, Pin::Positive),
        ));
        assert!(matches!(
            dmux(Pin::Negative, Pin::Negative),
            (Pin::Negative, Pin::Negative),
        ));
    }

    #[test]
    fn dmux_mux_is_id() {
        for x in [Pin::Positive, Pin::Negative] {
            for sel in [Pin::Positive, Pin::Negative] {
                let (a, b) = dmux(x, sel);
                assert_pin_equals!(mux(a, b, sel), x);
            }
        }
    }
}

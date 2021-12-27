use super::primitive::Bit;

pub fn not(x: Bit) -> Bit {
    x.nand(Bit::Positive)
}

pub fn and(x: Bit, y: Bit) -> Bit {
    not(x.nand(y))
}

pub fn or(x: Bit, y: Bit) -> Bit {
    not(x).nand(not(y))
}

pub fn xor(x: Bit, y: Bit) -> Bit {
    and(or(x, y), x.nand(y))
}

pub fn mux(x: Bit, y: Bit, sel: Bit) -> Bit {
    or(and(sel, x), and(not(sel), y))
}

pub fn dmux(x: Bit, sel: Bit) -> (Bit, Bit) {
    (and(x, sel), and(x, not(sel)))
}

#[cfg(test)]
mod test {
    use super::super::assert_bit_equals;
    use super::*;

    #[test]
    fn not_works() {
        assert!(matches!(not(Bit::Positive), Bit::Negative));
        assert!(matches!(not(Bit::Negative), Bit::Positive));
    }

    #[test]
    fn not_not_is_id() {
        for x in [Bit::Positive, Bit::Negative] {
            assert_bit_equals!(not(not(x)), x);
        }
    }

    #[test]
    fn and_works() {
        assert!(matches!(and(Bit::Positive, Bit::Positive), Bit::Positive));
        assert!(matches!(and(Bit::Positive, Bit::Negative), Bit::Negative));
        assert!(matches!(and(Bit::Negative, Bit::Positive), Bit::Negative));
        assert!(matches!(and(Bit::Negative, Bit::Negative), Bit::Negative));
    }

    #[test]
    fn or_works() {
        assert!(matches!(or(Bit::Positive, Bit::Positive), Bit::Positive));
        assert!(matches!(or(Bit::Positive, Bit::Negative), Bit::Positive));
        assert!(matches!(or(Bit::Negative, Bit::Positive), Bit::Positive));
        assert!(matches!(or(Bit::Negative, Bit::Negative), Bit::Negative));
    }

    #[test]
    fn xor_works() {
        assert!(matches!(xor(Bit::Positive, Bit::Positive), Bit::Negative));
        assert!(matches!(xor(Bit::Positive, Bit::Negative), Bit::Positive));
        assert!(matches!(xor(Bit::Negative, Bit::Positive), Bit::Positive));
        assert!(matches!(xor(Bit::Negative, Bit::Negative), Bit::Negative));
    }

    #[test]
    fn mux_work() {
        for b in [Bit::Positive, Bit::Negative] {
            assert!(matches!(
                mux(Bit::Positive, b, Bit::Positive),
                Bit::Positive,
            ));
            assert!(matches!(
                mux(Bit::Negative, b, Bit::Positive),
                Bit::Negative,
            ));
        }
        for a in [Bit::Positive, Bit::Negative] {
            assert!(matches!(
                mux(a, Bit::Positive, Bit::Negative),
                Bit::Positive,
            ));
            assert!(matches!(
                mux(a, Bit::Negative, Bit::Negative),
                Bit::Negative,
            ));
        }
    }

    #[test]
    fn dmux_work() {
        assert!(matches!(
            dmux(Bit::Positive, Bit::Positive),
            (Bit::Positive, Bit::Negative),
        ));
        assert!(matches!(
            dmux(Bit::Negative, Bit::Positive),
            (Bit::Negative, Bit::Negative),
        ));
        assert!(matches!(
            dmux(Bit::Positive, Bit::Negative),
            (Bit::Negative, Bit::Positive),
        ));
        assert!(matches!(
            dmux(Bit::Negative, Bit::Negative),
            (Bit::Negative, Bit::Negative),
        ));
    }

    #[test]
    fn dmux_mux_is_id() {
        for x in [Bit::Positive, Bit::Negative] {
            for sel in [Bit::Positive, Bit::Negative] {
                let (a, b) = dmux(x, sel);
                assert_bit_equals!(mux(a, b, sel), x);
            }
        }
    }
}

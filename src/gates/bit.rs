use crate::gates::bus2::Bus2;
use crate::gates::bus3::Bus3;
use crate::primitive::Bit;

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
    or(and(not(sel), x), and(sel, y))
}

pub fn dmux(x: Bit, sel: Bit) -> (Bit, Bit) {
    (and(x, not(sel)), and(x, sel))
}

pub fn dmux4way(x: Bit, sel: &Bus2) -> (Bit, Bit, Bit, Bit) {
    let (u, v) = dmux(x, sel.0[0]);
    let (a, b) = dmux(u, sel.0[1]);
    let (c, d) = dmux(v, sel.0[1]);
    (a, b, c, d)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_bit_equals;

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
                mux(Bit::Positive, b, Bit::Negative),
                Bit::Positive,
            ));
            assert!(matches!(
                mux(Bit::Negative, b, Bit::Negative),
                Bit::Negative,
            ));
        }
        for a in [Bit::Positive, Bit::Negative] {
            assert!(matches!(
                mux(a, Bit::Positive, Bit::Positive),
                Bit::Positive,
            ));
            assert!(matches!(
                mux(a, Bit::Negative, Bit::Positive),
                Bit::Negative,
            ));
        }
    }

    #[test]
    fn dmux_work() {
        assert!(matches!(
            dmux(Bit::Positive, Bit::Positive),
            (Bit::Negative, Bit::Positive),
        ));
        assert!(matches!(
            dmux(Bit::Negative, Bit::Positive),
            (Bit::Negative, Bit::Negative),
        ));
        assert!(matches!(
            dmux(Bit::Positive, Bit::Negative),
            (Bit::Positive, Bit::Negative),
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

    #[test]
    fn dmux4way_works() {
        for x in [Bit::Negative, Bit::Positive] {
            {
                let y = dmux4way(x, &Bus2([Bit::Negative, Bit::Negative]));
                assert_bit_equals!(y.0, x);
                assert_bit_equals!(y.1, Bit::Negative);
                assert_bit_equals!(y.2, Bit::Negative);
                assert_bit_equals!(y.3, Bit::Negative);
            }
            {
                let y = dmux4way(x, &Bus2([Bit::Negative, Bit::Positive]));
                assert_bit_equals!(y.0, Bit::Negative);
                assert_bit_equals!(y.1, x);
                assert_bit_equals!(y.2, Bit::Negative);
                assert_bit_equals!(y.3, Bit::Negative);
            }
            {
                let y = dmux4way(x, &Bus2([Bit::Positive, Bit::Negative]));
                assert_bit_equals!(y.0, Bit::Negative);
                assert_bit_equals!(y.1, Bit::Negative);
                assert_bit_equals!(y.2, x);
                assert_bit_equals!(y.3, Bit::Negative);
            }
            {
                let y = dmux4way(x, &Bus2([Bit::Positive, Bit::Positive]));
                assert_bit_equals!(y.0, Bit::Negative);
                assert_bit_equals!(y.1, Bit::Negative);
                assert_bit_equals!(y.2, Bit::Negative);
                assert_bit_equals!(y.3, x);
            }
        }
    }
}

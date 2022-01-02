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

pub fn dmux(x: Bit, sel: Bit) -> [Bit; 2] {
    [and(x, not(sel)), and(x, sel)]
}

pub fn dmux4way(x: Bit, sel: &Bus2) -> [Bit; 4] {
    let [u, v] = dmux(x, sel[0]);
    let [a, b] = dmux(u, sel[1]);
    let [c, d] = dmux(v, sel[1]);
    [a, b, c, d]
}

pub fn dmux8way(x: Bit, sel: &Bus3) -> [Bit; 8] {
    let bus2 = [sel[0], sel[1]];
    let [s, t, u, v] = dmux4way(x, &bus2);
    let [a, b] = dmux(s, sel[2]);
    let [c, d] = dmux(t, sel[2]);
    let [e, f] = dmux(u, sel[2]);
    let [g, h] = dmux(v, sel[2]);
    [a, b, c, d, e, f, g, h]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_bit_equals;
    use crate::gates::bus3::testing::make_bus3;

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
            [Bit::Negative, Bit::Positive],
        ));
        assert!(matches!(
            dmux(Bit::Negative, Bit::Positive),
            [Bit::Negative, Bit::Negative],
        ));
        assert!(matches!(
            dmux(Bit::Positive, Bit::Negative),
            [Bit::Positive, Bit::Negative],
        ));
        assert!(matches!(
            dmux(Bit::Negative, Bit::Negative),
            [Bit::Negative, Bit::Negative],
        ));
    }

    #[test]
    fn dmux_mux_is_id() {
        for x in [Bit::Positive, Bit::Negative] {
            for sel in [Bit::Positive, Bit::Negative] {
                let [a, b] = dmux(x, sel);
                assert_bit_equals!(mux(a, b, sel), x);
            }
        }
    }

    #[test]
    fn dmux4way_works() {
        for x in [Bit::Negative, Bit::Positive] {
            for i in 0..4 {
                let sel = make_bus2(i);
                let y = dmux4way(x, &sel);
                for j in 0..4 {
                    if i == j {
                        assert_bit_equals!(y[j], x);
                    } else {
                        assert_bit_equals!(y[j], Bit::Negative);
                    }
                }
            }
        }
    }

    #[test]
    fn dmux8way_works() {
        for x in [Bit::Negative, Bit::Positive] {
            for i in 0..8 {
                let sel = make_bus3(i as i32);
                let y = dmux8way(x, &sel);
                for j in 0..8 {
                    if i == j {
                        assert_bit_equals!(y[j], x);
                    } else {
                        assert_bit_equals!(y[j], Bit::Negative);
                    }
                }
            }
        }
    }

    fn make_bus2(i: usize) -> Bus2 {
        [
            if (i >> 1) % 2 == 0 {
                Bit::Negative
            } else {
                Bit::Positive
            },
            if i % 2 == 0 {
                Bit::Negative
            } else {
                Bit::Positive
            },
        ]
    }
}

pub mod testing {
    use super::*;

    pub fn make_bit(b: bool) -> Bit {
        if b {
            Bit::Positive
        } else {
            Bit::Negative
        }
    }
}

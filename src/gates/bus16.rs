use crate::gates::bus2::Bus2;
use crate::gates::bus3::Bus3;
use crate::primitive::Bit;

use super::bit;

#[derive(Clone, Debug)]
pub struct Bus16(pub [Bit; 16]);

#[macro_export]
macro_rules! assert_bus16_equals {
    ($actual:expr, $expected:expr) => {
        let Bus16(a) = $actual;
        let Bus16(b) = $expected;
        for i in 0..16 {
            assert_bit_equals!(a[i], b[i], format!("bit {} is different", i));
        }
    };
    ($actual:expr, $expected:expr, $arg:expr) => {
        let Bus16(a) = $actual;
        let Bus16(b) = $expected;
        for i in 0..16 {
            assert_bit_equals!(a[i], b[i], format!("{}, bit {} is different", $arg, i));
        }
    };
}

pub fn not(x: &Bus16) -> Bus16 {
    Bus16([
        bit::not(x.0[0]),
        bit::not(x.0[1]),
        bit::not(x.0[2]),
        bit::not(x.0[3]),
        bit::not(x.0[4]),
        bit::not(x.0[5]),
        bit::not(x.0[6]),
        bit::not(x.0[7]),
        bit::not(x.0[8]),
        bit::not(x.0[9]),
        bit::not(x.0[10]),
        bit::not(x.0[11]),
        bit::not(x.0[12]),
        bit::not(x.0[13]),
        bit::not(x.0[14]),
        bit::not(x.0[15]),
    ])
}

pub fn and(x: &Bus16, y: &Bus16) -> Bus16 {
    Bus16([
        bit::and(x.0[0], y.0[0]),
        bit::and(x.0[1], y.0[1]),
        bit::and(x.0[2], y.0[2]),
        bit::and(x.0[3], y.0[3]),
        bit::and(x.0[4], y.0[4]),
        bit::and(x.0[5], y.0[5]),
        bit::and(x.0[6], y.0[6]),
        bit::and(x.0[7], y.0[7]),
        bit::and(x.0[8], y.0[8]),
        bit::and(x.0[9], y.0[9]),
        bit::and(x.0[10], y.0[10]),
        bit::and(x.0[11], y.0[11]),
        bit::and(x.0[12], y.0[12]),
        bit::and(x.0[13], y.0[13]),
        bit::and(x.0[14], y.0[14]),
        bit::and(x.0[15], y.0[15]),
    ])
}

pub fn or(x: &Bus16, y: &Bus16) -> Bus16 {
    Bus16([
        bit::or(x.0[0], y.0[0]),
        bit::or(x.0[1], y.0[1]),
        bit::or(x.0[2], y.0[2]),
        bit::or(x.0[3], y.0[3]),
        bit::or(x.0[4], y.0[4]),
        bit::or(x.0[5], y.0[5]),
        bit::or(x.0[6], y.0[6]),
        bit::or(x.0[7], y.0[7]),
        bit::or(x.0[8], y.0[8]),
        bit::or(x.0[9], y.0[9]),
        bit::or(x.0[10], y.0[10]),
        bit::or(x.0[11], y.0[11]),
        bit::or(x.0[12], y.0[12]),
        bit::or(x.0[13], y.0[13]),
        bit::or(x.0[14], y.0[14]),
        bit::or(x.0[15], y.0[15]),
    ])
}

pub fn broadcast(x: Bit) -> Bus16 {
    Bus16([x, x, x, x, x, x, x, x, x, x, x, x, x, x, x, x])
}

pub fn mux(x: &Bus16, y: &Bus16, sel: Bit) -> Bus16 {
    Bus16([
        bit::mux(x.0[0], y.0[0], sel),
        bit::mux(x.0[1], y.0[1], sel),
        bit::mux(x.0[2], y.0[2], sel),
        bit::mux(x.0[3], y.0[3], sel),
        bit::mux(x.0[4], y.0[4], sel),
        bit::mux(x.0[5], y.0[5], sel),
        bit::mux(x.0[6], y.0[6], sel),
        bit::mux(x.0[7], y.0[7], sel),
        bit::mux(x.0[8], y.0[8], sel),
        bit::mux(x.0[9], y.0[9], sel),
        bit::mux(x.0[10], y.0[10], sel),
        bit::mux(x.0[11], y.0[11], sel),
        bit::mux(x.0[12], y.0[12], sel),
        bit::mux(x.0[13], y.0[13], sel),
        bit::mux(x.0[14], y.0[14], sel),
        bit::mux(x.0[15], y.0[15], sel),
    ])
}

pub fn mux4way16(a: &Bus16, b: &Bus16, c: &Bus16, d: &Bus16, sel: &Bus2) -> Bus16 {
    let s = mux(a, b, sel[1]);
    let t = mux(c, d, sel[1]);
    mux(&s, &t, sel[0])
}

pub fn mux8way16(
    a: &Bus16,
    b: &Bus16,
    c: &Bus16,
    d: &Bus16,
    e: &Bus16,
    f: &Bus16,
    g: &Bus16,
    h: &Bus16,
    sel: &Bus3,
) -> Bus16 {
    let bus2 = [sel[1], sel[2]];
    let s = mux4way16(a, b, c, d, &bus2);
    let t = mux4way16(e, f, g, h, &bus2);
    mux(&s, &t, sel[0])
}

pub fn dmux(x: &Bus16, sel: Bit) -> [Bus16; 2] {
    let not_sel = bit::not(sel);
    [and(x, &broadcast(not_sel)), and(x, &broadcast(sel))]
}

pub fn dmux4way16(x: &Bus16, sel: &Bus2) -> [Bus16; 4] {
    let [s0, s1] = sel;
    let [u, v] = dmux(x, *s0);
    let [a, b] = dmux(&u, *s1);
    let [c, d] = dmux(&v, *s1);
    [a, b, c, d]
}

pub fn dmux8way16(x: &Bus16, sel: &Bus3) -> [Bus16; 8] {
    let b2 = [sel[0], sel[1]];
    let [s, t, u, v] = dmux4way16(x, &b2);
    let [a, b] = dmux(&s, sel[2]);
    let [c, d] = dmux(&t, sel[2]);
    let [e, f] = dmux(&u, sel[2]);
    let [g, h] = dmux(&v, sel[2]);
    [a, b, c, d, e, f, g, h]
}

pub fn or16way(x: &Bus16) -> Bit {
    bit::or(
        bit::or(
            bit::or(bit::or(x.0[0], x.0[1]), bit::or(x.0[2], x.0[3])),
            bit::or(bit::or(x.0[4], x.0[5]), bit::or(x.0[6], x.0[7])),
        ),
        bit::or(
            bit::or(bit::or(x.0[8], x.0[9]), bit::or(x.0[10], x.0[11])),
            bit::or(bit::or(x.0[12], x.0[13]), bit::or(x.0[14], x.0[15])),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::testing::make_bus16;
    use super::*;
    use crate::assert_bit_equals;
    use crate::assert_bus16_equals;
    use crate::gates::bus3::testing::make_bus3;
    use crate::primitive::Bit;

    const FXT: Bus16 = Bus16([
        Bit::Negative,
        Bit::Positive,
        Bit::Negative,
        Bit::Negative,
        Bit::Positive,
        Bit::Positive,
        Bit::Negative,
        Bit::Negative,
        Bit::Negative,
        Bit::Positive,
        Bit::Positive,
        Bit::Positive,
        Bit::Negative,
        Bit::Negative,
        Bit::Positive,
        Bit::Positive,
    ]);

    #[test]
    fn assert_bus16_equals_works() {
        assert_bus16_equals!(Bus16([Bit::Positive; 16]), Bus16([Bit::Positive; 16]));
        assert_bus16_equals!(Bus16([Bit::Negative; 16]), Bus16([Bit::Negative; 16]));
    }

    #[test]
    fn not_works() {
        assert_bus16_equals!(not(&Bus16([Bit::Positive; 16])), Bus16([Bit::Negative; 16]));
        assert_bus16_equals!(not(&Bus16([Bit::Negative; 16])), Bus16([Bit::Positive; 16]));
        assert_bus16_equals!(
            not(&FXT),
            Bus16([
                Bit::Positive,
                Bit::Negative,
                Bit::Positive,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Positive,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
            ])
        );
    }

    #[test]
    fn and_works() {
        assert_bus16_equals!(and(&FXT, &Bus16([Bit::Positive; 16])), &FXT);
        assert_bus16_equals!(
            and(&FXT, &Bus16([Bit::Negative; 16])),
            Bus16([Bit::Negative; 16])
        );
    }

    #[test]
    fn or_works() {
        assert_bus16_equals!(
            or(&FXT, &Bus16([Bit::Positive; 16])),
            Bus16([Bit::Positive; 16])
        );
        assert_bus16_equals!(or(&FXT, &Bus16([Bit::Negative; 16])), &FXT);
    }

    #[test]
    fn mux_works() {
        assert_bus16_equals!(mux(&FXT, &Bus16([Bit::Positive; 16]), Bit::Negative), &FXT);
        assert_bus16_equals!(
            mux(&FXT, &Bus16([Bit::Positive; 16]), Bit::Positive),
            &Bus16([Bit::Positive; 16])
        );
    }

    #[test]
    fn dmux_works() {
        let [a, b] = dmux(&FXT, Bit::Negative);
        assert_bus16_equals!(&a, &FXT);
        assert_bus16_equals!(&b, &Bus16([Bit::Negative; 16]));

        let [a, b] = dmux(&FXT, Bit::Positive);
        assert_bus16_equals!(&a, &Bus16([Bit::Negative; 16]));
        assert_bus16_equals!(&b, &FXT);
    }

    #[test]
    fn mux4way16_works() {
        let h = Bus16([
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
        ]);
        let p = Bus16([Bit::Positive; 16]);
        let n = Bus16([Bit::Negative; 16]);

        assert_bus16_equals!(
            mux4way16(&FXT, &h, &p, &n, &[Bit::Negative, Bit::Negative]),
            &FXT
        );
        assert_bus16_equals!(
            mux4way16(&FXT, &h, &p, &n, &[Bit::Negative, Bit::Positive]),
            &h
        );
        assert_bus16_equals!(
            mux4way16(&FXT, &h, &p, &n, &[Bit::Positive, Bit::Negative]),
            &p
        );
        assert_bus16_equals!(
            mux4way16(&FXT, &h, &p, &n, &[Bit::Positive, Bit::Positive]),
            &n
        );
    }

    #[test]
    fn mux8way16_works() {
        let h = Bus16([
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
            Bit::Positive,
        ]);
        let p = Bus16([Bit::Positive; 16]);
        let n = Bus16([Bit::Negative; 16]);

        assert_bus16_equals!(
            mux8way16(
                &FXT,
                &h,
                &p,
                &n,
                &n,
                &n,
                &n,
                &n,
                &[Bit::Negative, Bit::Negative, Bit::Negative]
            ),
            &FXT
        );
        assert_bus16_equals!(
            mux8way16(
                &FXT,
                &n,
                &p,
                &n,
                &n,
                &h,
                &n,
                &n,
                &[Bit::Positive, Bit::Negative, Bit::Positive]
            ),
            &h
        );
        assert_bus16_equals!(
            mux8way16(
                &n,
                &h,
                &FXT,
                &n,
                &n,
                &n,
                &n,
                &n,
                &[Bit::Negative, Bit::Positive, Bit::Negative]
            ),
            &FXT
        );
    }

    #[test]
    fn dmux8way16_works() {
        let x = make_bus16(1010);
        for i in 0..8 {
            let sel = make_bus3(i as i32);
            let out = dmux8way16(&x, &sel);
            for j in 0..8 {
                if i == j {
                    assert_bus16_equals!(&out[j], &x);
                } else {
                    assert_bus16_equals!(&out[j], &make_bus16(0));
                }
            }
        }
    }

    #[test]
    fn or16way_works() {
        assert_bit_equals!(or16way(&Bus16([Bit::Negative; 16])), Bit::Negative);
        for i in 0..15 {
            let mut bits = [Bit::Negative; 16];
            bits[i] = Bit::Positive;
            assert_bit_equals!(or16way(&Bus16(bits)), Bit::Positive);
        }
    }
}

pub mod testing {
    use super::*;
    use crate::gates::bit::testing::make_bit;

    pub fn make_bus16(i: i32) -> Bus16 {
        let mut b16 = [Bit::Negative; 16];
        for b in 0..16 {
            b16[b] = make_bit(i & (1 << (15 - b)) != 0);
        }
        Bus16(b16)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::assert_bit_equals;
        use crate::assert_bus16_equals;

        #[test]
        fn make_bus16_works() {
            let a = make_bus16(0);
            assert_bus16_equals!(a, Bus16([Bit::Negative; 16]), format!("{:?}", a));
        }
    }
}

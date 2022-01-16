use crate::gates::bus2::Bus2;
use crate::gates::bus3::Bus3;
use crate::primitive::Bit;

use super::bit;

pub type Bus16 = [Bit; 16];

#[macro_export]
macro_rules! assert_bus16_equals {
    ($actual:expr, $expected:expr) => {
        for i in 0..16 {
            assert_bit_equals!($actual[i], $expected[i], format!("bit {} is different", i));
        }
    };
    ($actual:expr, $expected:expr, $arg:expr) => {
        for i in 0..16 {
            assert_bit_equals!(
                $actual[i],
                $expected[i],
                format!("{}, bit {} is different", $arg, i)
            );
        }
    };
}

pub fn not(x: &Bus16) -> Bus16 {
    [
        bit::not(x[0]),
        bit::not(x[1]),
        bit::not(x[2]),
        bit::not(x[3]),
        bit::not(x[4]),
        bit::not(x[5]),
        bit::not(x[6]),
        bit::not(x[7]),
        bit::not(x[8]),
        bit::not(x[9]),
        bit::not(x[10]),
        bit::not(x[11]),
        bit::not(x[12]),
        bit::not(x[13]),
        bit::not(x[14]),
        bit::not(x[15]),
    ]
}

pub fn and(x: &Bus16, y: &Bus16) -> Bus16 {
    [
        bit::and(x[0], y[0]),
        bit::and(x[1], y[1]),
        bit::and(x[2], y[2]),
        bit::and(x[3], y[3]),
        bit::and(x[4], y[4]),
        bit::and(x[5], y[5]),
        bit::and(x[6], y[6]),
        bit::and(x[7], y[7]),
        bit::and(x[8], y[8]),
        bit::and(x[9], y[9]),
        bit::and(x[10], y[10]),
        bit::and(x[11], y[11]),
        bit::and(x[12], y[12]),
        bit::and(x[13], y[13]),
        bit::and(x[14], y[14]),
        bit::and(x[15], y[15]),
    ]
}

pub fn or(x: &Bus16, y: &Bus16) -> Bus16 {
    [
        bit::or(x[0], y[0]),
        bit::or(x[1], y[1]),
        bit::or(x[2], y[2]),
        bit::or(x[3], y[3]),
        bit::or(x[4], y[4]),
        bit::or(x[5], y[5]),
        bit::or(x[6], y[6]),
        bit::or(x[7], y[7]),
        bit::or(x[8], y[8]),
        bit::or(x[9], y[9]),
        bit::or(x[10], y[10]),
        bit::or(x[11], y[11]),
        bit::or(x[12], y[12]),
        bit::or(x[13], y[13]),
        bit::or(x[14], y[14]),
        bit::or(x[15], y[15]),
    ]
}

pub fn broadcast(x: Bit) -> Bus16 {
    [x, x, x, x, x, x, x, x, x, x, x, x, x, x, x, x]
}

pub fn mux(x: &Bus16, y: &Bus16, sel: Bit) -> Bus16 {
    [
        bit::mux(x[0], y[0], sel),
        bit::mux(x[1], y[1], sel),
        bit::mux(x[2], y[2], sel),
        bit::mux(x[3], y[3], sel),
        bit::mux(x[4], y[4], sel),
        bit::mux(x[5], y[5], sel),
        bit::mux(x[6], y[6], sel),
        bit::mux(x[7], y[7], sel),
        bit::mux(x[8], y[8], sel),
        bit::mux(x[9], y[9], sel),
        bit::mux(x[10], y[10], sel),
        bit::mux(x[11], y[11], sel),
        bit::mux(x[12], y[12], sel),
        bit::mux(x[13], y[13], sel),
        bit::mux(x[14], y[14], sel),
        bit::mux(x[15], y[15], sel),
    ]
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
            bit::or(bit::or(x[0], x[1]), bit::or(x[2], x[3])),
            bit::or(bit::or(x[4], x[5]), bit::or(x[6], x[7])),
        ),
        bit::or(
            bit::or(bit::or(x[8], x[9]), bit::or(x[10], x[11])),
            bit::or(bit::or(x[12], x[13]), bit::or(x[14], x[15])),
        ),
    )
}

pub fn into_bus15(x: &Bus16) -> [Bit; 15] {
    [
        x[0x1], x[0x2], x[0x3], x[0x4], x[0x5], x[0x6], x[0x7], x[0x8], x[0x9], x[0xa], x[0xb],
        x[0xc], x[0xd], x[0xe], x[0xf],
    ]
}

pub fn into_bus13(x: &Bus16) -> [Bit; 13] {
    [
        x[0x3], x[0x4], x[0x5], x[0x6], x[0x7], x[0x8], x[0x9], x[0xa], x[0xb], x[0xc], x[0xd],
        x[0xe], x[0xf],
    ]
}

#[cfg(test)]
mod tests {
    use super::testing::make_bus16;
    use super::*;
    use crate::assert_bit_equals;
    use crate::assert_bus16_equals;
    use crate::gates::bus3::testing::make_bus3;
    use crate::primitive::Bit;

    const FXT: Bus16 = [
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
    ];

    #[test]
    fn assert_bus16_equals_works() {
        assert_bus16_equals!([Bit::Positive; 16], [Bit::Positive; 16]);
        assert_bus16_equals!([Bit::Negative; 16], [Bit::Negative; 16]);
    }

    #[test]
    fn not_works() {
        assert_bus16_equals!(not(&[Bit::Positive; 16]), [Bit::Negative; 16]);
        assert_bus16_equals!(not(&[Bit::Negative; 16]), [Bit::Positive; 16]);
        assert_bus16_equals!(
            not(&FXT),
            [
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
            ]
        );
    }

    #[test]
    fn and_works() {
        assert_bus16_equals!(and(&FXT, &[Bit::Positive; 16]), &FXT);
        assert_bus16_equals!(and(&FXT, &[Bit::Negative; 16]), [Bit::Negative; 16]);
    }

    #[test]
    fn or_works() {
        assert_bus16_equals!(or(&FXT, &[Bit::Positive; 16]), [Bit::Positive; 16]);
        assert_bus16_equals!(or(&FXT, &[Bit::Negative; 16]), &FXT);
    }

    #[test]
    fn mux_works() {
        assert_bus16_equals!(mux(&FXT, &[Bit::Positive; 16], Bit::Negative), &FXT);
        assert_bus16_equals!(
            mux(&FXT, &[Bit::Positive; 16], Bit::Positive),
            &[Bit::Positive; 16]
        );
    }

    #[test]
    fn dmux_works() {
        let [a, b] = dmux(&FXT, Bit::Negative);
        assert_bus16_equals!(&a, &FXT);
        assert_bus16_equals!(&b, &[Bit::Negative; 16]);

        let [a, b] = dmux(&FXT, Bit::Positive);
        assert_bus16_equals!(&a, &[Bit::Negative; 16]);
        assert_bus16_equals!(&b, &FXT);
    }

    #[test]
    fn mux4way16_works() {
        let h = [
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
        ];
        let p = [Bit::Positive; 16];
        let n = [Bit::Negative; 16];

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
        let h = [
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
        ];
        let p = [Bit::Positive; 16];
        let n = [Bit::Negative; 16];

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
        assert_bit_equals!(or16way(&[Bit::Negative; 16]), Bit::Negative);
        for i in 0..15 {
            let mut bits = [Bit::Negative; 16];
            bits[i] = Bit::Positive;
            assert_bit_equals!(or16way(&bits), Bit::Positive);
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
        b16
    }

    pub fn into_i32(b: &Bus16) -> i32 {
        let mut a = 0;
        for i in 0..16 {
            match b[i] {
                Bit::Positive => {
                    a += 1 << (15 - i);
                }
                Bit::Negative => (),
            }
        }
        a
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::assert_bit_equals;
        use crate::assert_bus16_equals;

        #[test]
        fn make_bus16_works() {
            let a = make_bus16(0);
            assert_bus16_equals!(a, [Bit::Negative; 16], format!("{:?}", a));
        }

        #[test]
        fn into_i32_works() {
            for i in 0..10 {
                assert_eq!(into_i32(&make_bus16(i)), i);
            }
        }
    }
}

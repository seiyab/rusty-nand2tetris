use crate::gates::bit;
use crate::gates::bus16::{mux8way16, Bus16};
use crate::gates::bus3::Bus3;
use crate::general::T8;
use crate::infrastructure::sequential::{FeedforwardSC, FeedforwardSCDef, TupleSC2};
use crate::primitive::Bit;

use super::ram8::{Ram8, Ram8Input};

pub type R2 = TupleSC2<Ram8, Ram8>;
pub type R4 = TupleSC2<R2, R2>;
pub type R8 = TupleSC2<R4, R4>;

pub type Ram64 = FeedforwardSC<R8, Ram64Impl>;

pub struct Ram64Impl;

pub struct Ram64Input {
    pub input: Bus16,
    pub address: [Bit; 6],
    pub load: Bit,
}

impl FeedforwardSCDef<R8> for Ram64Impl {
    type Input = Ram64Input;
    type Output = Bus16;
    type Jump = Bus3;

    fn new() -> R8 {
        TupleSC2::new(
            TupleSC2::new(
                TupleSC2::new(Ram8::new(), Ram8::new()),
                TupleSC2::new(Ram8::new(), Ram8::new()),
            ),
            TupleSC2::new(
                TupleSC2::new(Ram8::new(), Ram8::new()),
                TupleSC2::new(Ram8::new(), Ram8::new()),
            ),
        )
    }
    fn pre(input: &Self::Input) -> (T8<Ram8Input>, Self::Jump) {
        let Ram64Input {
            input: i,
            address: a,
            load,
        } = input;
        let r = |i: &Bus16, load: Bit| Ram8Input {
            input: i.clone(),
            address: [a[3], a[4], a[5]],
            load,
        };
        let a00 = bit::and(bit::not(a[0]), bit::not(a[1]));
        let a01 = bit::and(bit::not(a[0]), a[1]);
        let a10 = bit::and(a[0], bit::not(a[1]));
        let a11 = bit::and(a[0], a[1]);
        let sel = [
            bit::and(a00, bit::not(a[2])),
            bit::and(a00, a[2]),
            bit::and(a01, bit::not(a[2])),
            bit::and(a01, a[2]),
            bit::and(a10, bit::not(a[2])),
            bit::and(a10, a[2]),
            bit::and(a11, bit::not(a[2])),
            bit::and(a11, a[2]),
        ];
        (
            (
                (
                    (
                        r(&i, bit::and(*load, sel[0])),
                        r(&i, bit::and(*load, sel[1])),
                    ),
                    (
                        r(&i, bit::and(*load, sel[2])),
                        r(&i, bit::and(*load, sel[3])),
                    ),
                ),
                (
                    (
                        r(&i, bit::and(*load, sel[4])),
                        r(&i, bit::and(*load, sel[5])),
                    ),
                    (
                        r(&i, bit::and(*load, sel[6])),
                        r(&i, bit::and(*load, sel[7])),
                    ),
                ),
            ),
            [a[0], a[1], a[2]],
        )
    }
    fn post(buf: &T8<Bus16>, jump: &Self::Jump) -> Self::Output {
        let sel = jump;
        let (((b0, b1), (b2, b3)), ((b4, b5), (b6, b7))) = buf;
        mux8way16(b0, b1, b2, b3, b4, b5, b6, b7, sel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gates::bus16::testing::*;
    use crate::gates::bus3::testing::*;
    use crate::infrastructure::sequential::SequentialCircuit;

    use crate::assert_bit_equals;
    use crate::assert_bus16_equals;

    #[test]
    fn ram64_works() {
        let mut r = Ram64::new();
        for i in 0..8 {
            let b3 = make_bus3(i);
            let (o, rr) = r.tick(&Ram64Input {
                input: make_bus16(i),
                address: [b3[0], b3[1], b3[2], b3[0], b3[1], b3[2]],
                load: Bit::Positive,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(0), format!("addr = {}, {:?}", i, o));
        }
        for i in 0..8 {
            let b3 = make_bus3(i);
            let (o, rr) = r.tick(&Ram64Input {
                input: make_bus16(i),
                address: [b3[0], b3[1], b3[2], b3[0], b3[1], b3[2]],
                load: Bit::Negative,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(i));
        }
        for i in 0..8 {
            let b3 = make_bus3(i);
            let (o, rr) = r.tick(&Ram64Input {
                input: make_bus16(-1),
                address: [b3[0], b3[1], b3[2], b3[0], b3[1], b3[2]],
                load: Bit::Positive,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(i));
        }
    }
}

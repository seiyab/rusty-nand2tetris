use crate::gates::bit;
use crate::gates::bus16::{mux8way16, Bus16};
use crate::gates::bus3::Bus3;
use crate::general::T8;
use crate::infrastructure::sequential::{FeedforwardSC, FeedforwardSCDef, TupleSC2};
use crate::primitive::Bit;

use super::ram512::{Ram512, Ram512Input};

pub type R2 = TupleSC2<Ram512, Ram512>;
pub type R4 = TupleSC2<R2, R2>;
pub type R8 = TupleSC2<R4, R4>;

pub type Ram4k = FeedforwardSC<Box<R8>, Ram4kImpl>;

pub struct Ram4kImpl;

pub struct Ram4kInput {
    pub input: Bus16,
    pub address: [Bit; 12],
    pub load: Bit,
}

impl FeedforwardSCDef<Box<R8>> for Ram4kImpl {
    type Input = Ram4kInput;
    type Output = Bus16;
    type Jump = Bus3;

    fn new() -> Box<R8> {
        let x = Box::new(TupleSC2::new(
            TupleSC2::new(
                TupleSC2::new(Ram512::new(), Ram512::new()),
                TupleSC2::new(Ram512::new(), Ram512::new()),
            ),
            TupleSC2::new(
                TupleSC2::new(Ram512::new(), Ram512::new()),
                TupleSC2::new(Ram512::new(), Ram512::new()),
            ),
        ));
        x
    }
    fn pre(input: &Self::Input) -> (T8<Ram512Input>, Self::Jump) {
        let Ram4kInput {
            input: i,
            address: a,
            load,
        } = input;
        let r = |i: &Bus16, load: Bit| Ram512Input {
            input: i.clone(),
            address: [a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11]],
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
    fn ram4k_works() {
        let mut r = Ram4k::new();
        for i in 0..8 {
            let b3 = make_bus3(i);
            let (o, rr) = r.tick(&Ram4kInput {
                input: make_bus16(i),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1],
                    b3[2],
                ],
                load: Bit::Positive,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(0), format!("addr = {}, {:?}", i, o));
        }
        for i in 0..8 {
            let b3 = make_bus3(i);
            let (o, rr) = r.tick(&Ram4kInput {
                input: make_bus16(i),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1],
                    b3[2],
                ],
                load: Bit::Negative,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(i));
        }
        for i in 0..8 {
            let b3 = make_bus3(i);
            let (o, rr) = r.tick(&Ram4kInput {
                input: make_bus16(-1),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1],
                    b3[2],
                ],
                load: Bit::Positive,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(i));
        }
    }
}

use crate::gates::bit;
use crate::gates::bus16::{mux8way16, Bus16};
use crate::gates::bus3::Bus3;
use crate::general::T8;
use crate::infrastructure::sequential::{FeedforwardSC, FeedforwardSCDef, MutSC, TupleSC2};
use crate::primitive::Bit;

use super::ram64::{MutRam64, Ram64, Ram64Input};

pub type R2 = TupleSC2<Ram64, Ram64>;
pub type R4 = TupleSC2<R2, R2>;
pub type R8 = TupleSC2<R4, R4>;

pub type Ram512 = FeedforwardSC<Box<R8>, Ram512Impl>;

pub struct Ram512Impl;

pub struct Ram512Input {
    pub input: Bus16,
    pub address: [Bit; 9],
    pub load: Bit,
}

impl FeedforwardSCDef<Box<R8>> for Ram512Impl {
    type Input = Ram512Input;
    type Output = Bus16;
    type Jump = Bus3;

    fn new() -> Box<R8> {
        let x = Box::new(TupleSC2::new(
            TupleSC2::new(
                TupleSC2::new(Ram64::new(), Ram64::new()),
                TupleSC2::new(Ram64::new(), Ram64::new()),
            ),
            TupleSC2::new(
                TupleSC2::new(Ram64::new(), Ram64::new()),
                TupleSC2::new(Ram64::new(), Ram64::new()),
            ),
        ));
        x
    }
    fn pre(input: &Self::Input) -> (T8<Ram64Input>, Self::Jump) {
        let Ram512Input {
            input: i,
            address: a,
            load,
        } = input;
        let r = |i: &Bus16, load: Bit| Ram64Input {
            input: i.clone(),
            address: [a[3], a[4], a[5], a[6], a[7], a[8]],
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

pub struct MutRam512(Box<[MutRam64; 8]>);

impl MutRam512 {
    fn new() -> Self {
        Self(Box::new([
            MutRam64::new(),
            MutRam64::new(),
            MutRam64::new(),
            MutRam64::new(),
            MutRam64::new(),
            MutRam64::new(),
            MutRam64::new(),
            MutRam64::new(),
        ]))
    }
}

impl MutSC for MutRam512 {
    type Input = Ram512Input;
    type Output = Bus16;

    fn tick(&mut self, input: &Self::Input) -> Self::Output {
        let Ram512Input {
            address: a,
            input,
            load,
        } = input;
        let addr_high = [a[0], a[1], a[2]];
        let addr_low = [a[3], a[4], a[5], a[6], a[7], a[8]];
        let ram64input = Ram64Input {
            input: input.clone(),
            address: addr_low,
            load: *load,
        };
        match addr_high {
            [Bit::Negative, Bit::Negative, Bit::Negative] => self.0[0].tick(&ram64input),
            [Bit::Negative, Bit::Negative, Bit::Positive] => self.0[1].tick(&ram64input),
            [Bit::Negative, Bit::Positive, Bit::Negative] => self.0[2].tick(&ram64input),
            [Bit::Negative, Bit::Positive, Bit::Positive] => self.0[3].tick(&ram64input),
            [Bit::Positive, Bit::Negative, Bit::Negative] => self.0[4].tick(&ram64input),
            [Bit::Positive, Bit::Negative, Bit::Positive] => self.0[5].tick(&ram64input),
            [Bit::Positive, Bit::Positive, Bit::Negative] => self.0[6].tick(&ram64input),
            [Bit::Positive, Bit::Positive, Bit::Positive] => self.0[7].tick(&ram64input),
        }
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
    fn ram512_works() {
        let mut r = Ram512::new();
        for i in 0..8 {
            let b3 = make_bus3(i);
            let (o, rr) = r.tick(&Ram512Input {
                input: make_bus16(i),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2],
                ],
                load: Bit::Positive,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(0), format!("addr = {}, {:?}", i, o));
        }
        for i in 0..8 {
            let b3 = make_bus3(i);
            let (o, rr) = r.tick(&Ram512Input {
                input: make_bus16(i),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2],
                ],
                load: Bit::Negative,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(i));
        }
        for i in 0..8 {
            let b3 = make_bus3(i);
            let (o, rr) = r.tick(&Ram512Input {
                input: make_bus16(-1),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2],
                ],
                load: Bit::Positive,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(i));
        }
    }

    #[test]
    fn mut_ram512_works() {
        let mut r = MutRam512::new();
        for i in 0..8 {
            let b3 = make_bus3(i);
            let o = r.tick(&Ram512Input {
                input: make_bus16(i),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2],
                ],
                load: Bit::Positive,
            });
            assert_bus16_equals!(o, make_bus16(0), format!("addr = {}, {:?}", i, o));
        }
        for i in 0..8 {
            let b3 = make_bus3(i);
            let o = r.tick(&Ram512Input {
                input: make_bus16(i),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2],
                ],
                load: Bit::Negative,
            });
            assert_bus16_equals!(o, make_bus16(i));
        }
        for i in 0..8 {
            let b3 = make_bus3(i);
            let o = r.tick(&Ram512Input {
                input: make_bus16(-1),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2],
                ],
                load: Bit::Positive,
            });
            assert_bus16_equals!(o, make_bus16(i));
        }
    }
}

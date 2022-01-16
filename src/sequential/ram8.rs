use crate::gates::bit;
use crate::gates::bus16::{mux8way16, Bus16};
use crate::gates::bus3::Bus3;
use crate::general::Zero;
use crate::infrastructure::sequential::{ArraySC8, FeedforwardSC, FeedforwardSCDef};
use crate::primitive::Bit;

use super::register::{Register16, Register16Input};

pub type Ram8 = FeedforwardSC<ArraySC8<Register16>, Ram8Impl>;

pub struct Ram8Impl;

pub struct Ram8Input {
    pub input: Bus16,
    pub address: Bus3,
    pub load: Bit,
}

impl FeedforwardSCDef<ArraySC8<Register16>> for Ram8Impl {
    type Input = Ram8Input;
    type Output = Bus16;
    type Jump = Bus3;

    fn new() -> ArraySC8<Register16> {
        ArraySC8::new()
    }
    fn pre(input: &Self::Input) -> ([Register16Input; 8], Self::Jump) {
        let r = |i: &Bus16, load: Bit| Register16Input {
            input: i.clone(),
            load,
        };
        let Ram8Input {
            input: i,
            address: a,
            load,
        } = input;
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
            [
                r(&i, bit::and(*load, sel[0])),
                r(&i, bit::and(*load, sel[1])),
                r(&i, bit::and(*load, sel[2])),
                r(&i, bit::and(*load, sel[3])),
                r(&i, bit::and(*load, sel[4])),
                r(&i, bit::and(*load, sel[5])),
                r(&i, bit::and(*load, sel[6])),
                r(&i, bit::and(*load, sel[7])),
            ],
            a.clone(),
        )
    }
    fn post(buf: &[Bus16; 8], jump: &Self::Jump) -> Self::Output {
        let sel = jump;
        let [b0, b1, b2, b3, b4, b5, b6, b7] = buf;
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
    fn ram8_works() {
        let mut r = Ram8::new();
        for i in 0..8 {
            let (o, rr) = r.tick(&Ram8Input {
                input: make_bus16(i),
                address: make_bus3(i),
                load: Bit::Positive,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(0), format!("addr = {}, {:?}", i, o));
        }
        for i in 0..8 {
            let (o, rr) = r.tick(&Ram8Input {
                input: make_bus16(-1),
                address: make_bus3(i),
                load: Bit::Negative,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(i));
        }
        for i in 0..8 {
            let (o, rr) = r.tick(&Ram8Input {
                input: make_bus16(-1),
                address: make_bus3(i),
                load: Bit::Positive,
            });
            r = rr;
            assert_bus16_equals!(o, make_bus16(i));
        }
    }
}

pub mod testing {
    use super::Ram8;
    use crate::infrastructure::sequential::testing;

    impl Ram8 {
        pub fn peek_at(&self, i: usize) {
            self.peek().at(i);
        }
    }

}
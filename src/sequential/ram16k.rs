use crate::gates::bus16::Bus16;
use crate::general::Zero;
use crate::infrastructure::sequential::MutSC;
use crate::primitive::Bit;

use super::ram4k::{MutRam4k, Ram4kInput};

pub struct MutRam16k(Box<[MutRam4k; 4]>);

pub struct Ram16kInput {
    pub input: Bus16,
    pub address: [Bit; 14],
    pub load: Bit,
}

impl MutSC for MutRam16k {
    type Input = Ram16kInput;
    type Output = Bus16;

    fn tick(&mut self, input: &Self::Input) -> Self::Output {
        let Ram16kInput {
            address: a,
            input,
            load,
        } = input;
        let addr_high = [a[0], a[1]];
        let addr_low = [
            a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13],
        ];
        let ram4k_input = Ram4kInput {
            input: input.clone(),
            address: addr_low,
            load: *load,
        };
        match addr_high {
            [Bit::Negative, Bit::Negative] => self.0[0].tick(&ram4k_input),
            [Bit::Negative, Bit::Positive] => self.0[1].tick(&ram4k_input),
            [Bit::Positive, Bit::Negative] => self.0[2].tick(&ram4k_input),
            [Bit::Positive, Bit::Positive] => self.0[3].tick(&ram4k_input),
        }
    }
}

impl Zero for MutRam16k {
    fn new() -> Self {
        Self(Box::new([
            MutRam4k::new(),
            MutRam4k::new(),
            MutRam4k::new(),
            MutRam4k::new(),
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_bit_equals;
    use crate::assert_bus16_equals;
    use crate::gates::bus16::testing::make_bus16;
    use crate::gates::bus3::testing::make_bus3;

    #[test]
    fn mut_ram16k_works() {
        let mut r = MutRam16k::new();
        for i in 0..8 {
            let b3 = make_bus3(i);
            let o = r.tick(&Ram16kInput {
                input: make_bus16(i),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1],
                    b3[2], b3[0], b3[1],
                ],
                load: Bit::Positive,
            });
            assert_bus16_equals!(o, make_bus16(0), format!("addr = {}, {:?}", i, o));
        }
        for i in 0..8 {
            let b3 = make_bus3(i);
            let o = r.tick(&Ram16kInput {
                input: make_bus16(i),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1],
                    b3[2], b3[0], b3[1],
                ],
                load: Bit::Negative,
            });
            assert_bus16_equals!(o, make_bus16(i));
        }
        for i in 0..8 {
            let b3 = make_bus3(i);
            let o = r.tick(&Ram16kInput {
                input: make_bus16(-1),
                address: [
                    b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1], b3[2], b3[0], b3[1],
                    b3[2], b3[0], b3[1],
                ],
                load: Bit::Positive,
            });
            assert_bus16_equals!(o, make_bus16(i));
        }
    }
}

use crate::gates::bus16::Bus16;
use crate::general::Zero;
use crate::infrastructure::sequential::MutSC;
use crate::primitive::Bit;
use crate::sequential::{MutRam16k, MutRam4k, Ram16kInput, Ram4kInput};

// future work: should be constructed of gates
pub struct MutRom(Box<[MutRam16k; 2]>);

impl MutSC for MutRom {
    type Input = [Bit; 15];
    type Output = Bus16;

    fn tick(&mut self, a: &Self::Input) -> Self::Output {
        let addr_high = a[0];
        let addr_low = [
            a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14],
        ];
        let ram16k_input = Ram16kInput {
            input: Bus16::new(),
            address: addr_low,
            load: Bit::Negative,
        };
        match addr_high {
            Bit::Negative => self.0[0].tick(&ram16k_input),
            Bit::Positive => self.0[1].tick(&ram16k_input),
        }
    }
}

// future work: should be constructed of gates
pub struct MutDataMemory(Box<[MutRam4k; 2]>);

pub struct DataMemoryInput {
    pub input: Bus16,
    pub load: Bit,
    pub address: [Bit; 13],
}

impl MutSC for MutDataMemory {
    type Input = DataMemoryInput;
    type Output = Bus16;

    fn tick(&mut self, i: &Self::Input) -> Self::Output {
        let a = &i.address;
        let addr_high = a[0];
        let addr_low = [
            a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12],
        ];
        let ram4k_input = Ram4kInput {
            input: i.input.clone(),
            address: addr_low,
            load: i.load,
        };
        match addr_high {
            Bit::Negative => self.0[0].tick(&ram4k_input),
            Bit::Positive => self.0[1].tick(&ram4k_input),
        }
    }
}

impl Zero for MutDataMemory {
    fn new() -> Self {
        MutDataMemory(Box::new([MutRam4k::new(), MutRam4k::new()]))
    }
}

pub mod testing {
    use super::*;
    use crate::gates::bit::testing::*;

    impl MutRom {
        pub fn of(instructions: &Vec<Bus16>) -> Self {
            let mut ram0 = MutRam16k::new();
            let mut ram1 = MutRam16k::new();
            for (i, inst) in instructions.iter().enumerate() {
                let addr = make_bus15(i as i32);
                let addr14 = [
                    addr[0x1], addr[0x2], addr[0x3], addr[0x4], addr[0x5], addr[0x6], addr[0x7],
                    addr[0x8], addr[0x9], addr[0xa], addr[0xb], addr[0xc], addr[0xd], addr[0xe],
                ];
                match addr[0] {
                    Bit::Negative => {
                        ram0.tick(&Ram16kInput {
                            input: inst.clone(),
                            address: addr14,
                            load: Bit::Positive,
                        });
                    }
                    Bit::Positive => {
                        ram1.tick(&Ram16kInput {
                            input: inst.clone(),
                            address: addr14,
                            load: Bit::Positive,
                        });
                    }
                }
            }
            Self(Box::new([ram0, ram1]))
        }
    }

    impl MutDataMemory {
        pub fn peek(&mut self, addr: i32) -> Bus16 {
            self.tick(&DataMemoryInput {
                input: Bus16::new(),
                address: make_bus13(addr),
                load: Bit::Negative,
            })
        }
    }

    fn make_bus13(i: i32) -> [Bit; 13] {
        let mut b13 = [Bit::Negative; 13];
        for b in 0..13 {
            b13[b] = make_bit(i & (1 << (12 - b)) != 0);
        }
        b13
    }

    fn make_bus15(i: i32) -> [Bit; 15] {
        let mut b15 = [Bit::Negative; 15];
        for b in 0..15 {
            b15[b] = make_bit(i & (1 << (14 - b)) != 0);
        }
        b15
    }
}

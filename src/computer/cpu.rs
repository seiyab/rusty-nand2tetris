use crate::gates::adder::{alu, inc16, AluControl, AluOut};
use crate::gates::bit;
use crate::gates::bus16;
use crate::gates::bus16::Bus16;
use crate::general::Zero;
use crate::infrastructure::sequential::*;
use crate::primitive::Bit;
use crate::sequential::{Register16, Register16Input};

#[derive(Clone)]
pub struct CpuInput {
    pub in_m: Bus16,
    pub instruction: Bus16,
    pub reset: Bit,
}

pub struct CpuOutput {
    pub out_m: Bus16,
    pub write_m: Bit,
    pub address_m: Bus16,
    pub pc: [Bit; 15],
}

// data, address, program counter
pub type CpuRegisters = FeedbackSC<ArraySC3<Register16>, CpuRegistersDef>;
pub type Cpu = FeedforwardSC<CpuRegisters, CpuDef>;

pub struct CpuRegistersDef();

pub struct CpuRegisterOutput {
    out_m: Bus16,
    address_m: Bus16,
    pc: [Bit; 15],
}

impl FeedbackSCDef<ArraySC3<Register16>> for CpuRegistersDef {
    type Input = CpuInput;
    type Output = CpuRegisterOutput;
    type Feedback = (Bus16, Bus16); // out, pc

    fn pre(
        i: &Self::Input,
        f: &Self::Feedback,
    ) -> <ArraySC3<Register16> as SequentialCircuit>::Input {
        let c = i.instruction[0];
        let [d1, d2] = [i.instruction[10], i.instruction[11]];
        let pc = bus16::mux(&f.1, &Bus16::new(), i.reset);
        let a_in = bus16::mux(&i.instruction, &f.0, c);
        [
            Register16Input {
                input: f.0.clone(),
                load: bit::and(d2, c),
            },
            Register16Input {
                input: a_in,
                load: bit::or(d1, bit::not(c)),
            },
            Register16Input {
                input: pc,
                load: Bit::Positive,
            },
        ]
    }
    fn post(
        i: &Self::Input,
        b: &<ArraySC3<Register16> as SequentialCircuit>::Output,
    ) -> (Self::Output, Self::Feedback) {
        let c = i.instruction[0];
        let a = i.instruction[3];
        let y = bus16::mux(&b[1], &i.in_m, a);
        let AluOut { out, zr, ng } = alu(
            &b[0],
            &y,
            AluControl {
                zx: i.instruction[4],
                nx: i.instruction[5],
                zy: i.instruction[6],
                ny: i.instruction[7],
                f: i.instruction[8],
                no: i.instruction[9],
            },
        );
        let ps = bit::not(bit::or(zr, ng));
        let jj1 = bit::and(i.instruction[13], ng);
        let jj2 = bit::and(i.instruction[14], zr);
        let jj3 = bit::and(i.instruction[15], ps);
        let jump = bit::or(bit::or(jj1, jj2), jj3);
        let p = bus16::mux(&inc16(&b[2]), &b[1], bit::and(jump, c));
        (
            CpuRegisterOutput {
                out_m: out,
                address_m: b[1].clone(),
                pc: bus16::into_bus15(&b[2]),
            },
            (out, p),
        )
    }
}

pub struct CpuDef();

impl FeedforwardSCDef<CpuRegisters> for CpuDef {
    type Input = CpuInput;
    type Output = CpuOutput;
    type Jump = Bus16; // instruction

    fn new() -> CpuRegisters {
        CpuRegisters::new()
    }
    fn pre(i: &Self::Input) -> (<CpuRegisters as SequentialCircuit>::Input, Self::Jump) {
        (
            CpuInput {
                in_m: i.in_m.clone(),
                instruction: i.instruction.clone(),
                reset: i.reset,
            },
            i.instruction.clone(),
        )
    }
    fn post(b: &<CpuRegisters as SequentialCircuit>::Output, j: &Self::Jump) -> Self::Output {
        CpuOutput {
            out_m: b.out_m.clone(),
            write_m: bit::and(j[12], j[0]),
            address_m: b.address_m.clone(),
            pc: b.pc.clone(),
        }
    }
}

pub mod testing {
    use crate::instruction::*;

    use super::*;

    pub trait CpuDebug {
        fn peek_data(&self) -> Bus16;
        fn peek_address(&self) -> Bus16;
        fn peek_pc(&self) -> Bus16;
    }

    impl CpuDebug for Cpu {
        fn peek_data(&self) -> Bus16 {
            let (o, _) = self.tick(&CpuInput {
                in_m: Bus16::new(),
                instruction: Instruction::C(Computation {
                    comp: (CompReg::A, Comp::D),
                    dest: Dest::A,
                    jump: Jump::None,
                })
                .bus16(),
                reset: Bit::Negative,
            });
            o.out_m
        }

        fn peek_address(&self) -> Bus16 {
            let (o, _) = self.tick(&CpuInput {
                in_m: Bus16::new(),
                instruction: Instruction::C(Computation {
                    comp: (CompReg::A, Comp::A),
                    dest: Dest::A,
                    jump: Jump::None,
                })
                .bus16(),
                reset: Bit::Negative,
            });
            o.address_m
        }

        fn peek_pc(&self) -> Bus16 {
            let (o, _) = self.tick(&CpuInput {
                in_m: Bus16::new(),
                instruction: Instruction::C(Computation {
                    comp: (CompReg::A, Comp::A),
                    dest: Dest::A,
                    jump: Jump::None,
                })
                .bus16(),
                reset: Bit::Negative,
            });
            let p = o.pc;
            [
                Bit::Negative,
                p[0x0],
                p[0x1],
                p[0x2],
                p[0x3],
                p[0x4],
                p[0x5],
                p[0x6],
                p[0x7],
                p[0x8],
                p[0x9],
                p[0xa],
                p[0xb],
                p[0xc],
                p[0xd],
                p[0xe],
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::gates::bus16::testing::{into_i32, make_bus16};
    use crate::instruction::*;

    use super::testing::CpuDebug;

    #[test]
    fn load_a_works() {
        let c = Cpu::new();
        for i in 0..10 {
            let (o, c) = c.tick(&CpuInput {
                in_m: Bus16::new(),
                instruction: Instruction::A(i).bus16(),
                reset: Bit::Negative,
            });
            assert_eq!(into_i32(&c.peek_address()), i);
            assert_matches!(o.write_m, Bit::Negative);
        }
    }

    #[test]
    fn store_works() {
        let c = Cpu::new();
        let (_, c) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::A(123).bus16(),
            reset: Bit::Negative,
        });
        let (o, _) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
            reset: Bit::Negative,
        });
        assert_eq!(into_i32(&o.out_m), 123);
        assert_matches!(o.write_m, Bit::Positive);
    }

    #[test]
    fn a_plus_1_works() {
        let c = Cpu::new();
        let (_, c) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::A(11).bus16(),
            reset: Bit::Negative,
        });
        let (o, c) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::C(Computation {
                comp: (CompReg::A, Comp::APlus1),
                dest: Dest::A,
                jump: Jump::None,
            })
            .bus16(),
            reset: Bit::Negative,
        });
        assert_eq!(into_i32(&o.out_m), 12);

        let (o, _) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::None,
                jump: Jump::None,
            })
            .bus16(),
            reset: Bit::Negative,
        });
        assert_eq!(into_i32(&o.out_m), 12);
    }

    #[test]
    fn d_minus_a_works() {
        let c = Cpu::new();
        let (_, c) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::A(15).bus16(),
            reset: Bit::Negative,
        });
        let (_, c) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            reset: Bit::Negative,
        });
        let (_, c) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::A(10).bus16(),
            reset: Bit::Negative,
        });
        let (o, c) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::C(Computation {
                comp: (CompReg::A, Comp::DMinusA),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            reset: Bit::Negative,
        });
        assert_eq!(into_i32(&o.out_m), 5);

        let (o, _) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::None,
                jump: Jump::None,
            })
            .bus16(),
            reset: Bit::Negative,
        });
        assert_eq!(into_i32(&o.out_m), 5);
    }

    #[test]
    fn d_plus_m_works() {
        let c = Cpu::new();
        let (_, c) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::A(7).bus16(),
            reset: Bit::Negative,
        });
        let (_, c) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            reset: Bit::Negative,
        });
        let (o, c) = c.tick(&CpuInput {
            in_m: make_bus16(13),
            instruction: Instruction::C(Computation {
                comp: (CompReg::M, Comp::DPlusA),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            reset: Bit::Negative,
        });
        assert_eq!(into_i32(&o.out_m), 20);

        let (o, _) = c.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::None,
                jump: Jump::None,
            })
            .bus16(),
            reset: Bit::Negative,
        });
        assert_eq!(into_i32(&o.out_m), 20);
    }
}

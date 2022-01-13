use crate::gates::adder::{alu, inc16, AluControl, AluOut};
use crate::gates::bit;
use crate::gates::bus16;
use crate::gates::bus16::Bus16;
use crate::gates::bus3::Bus3;
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
type CpuRegisters = FeedbackSC<ArraySC3<Register16>, CpuRegistersDef>;
pub type Cpu = FeedbackSC<CpuRegisters, CpuDef>;

struct CpuRegistersDef();

struct CpuRegisterOutput {
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
        let [d2, d3] = [i.instruction[11], i.instruction[12]];
        let pc = bus16::mux(&f.1, &Bus16::new(), i.reset);
        let a_in = bus16::mux(&f.0, &i.instruction, c);
        [
            Register16Input {
                input: f.0.clone(),
                load: bit::and(d2, c),
            },
            Register16Input {
                input: a_in,
                load: bit::or(d3, bit::not(c)),
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
                out_m: out.clone(),
                address_m: b[1].clone(),
                pc: [
                    p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12],
                    p[13], p[14], p[15],
                ],
            },
            (out, p),
        )
    }
}

struct CpuDef();

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
            write_m: j[10],
            address_m: b.address_m.clone(),
            pc: b.pc.clone(),
        }
    }
}

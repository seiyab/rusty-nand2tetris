use crate::gates::bus16::Bus16;
use crate::primitive::Bit;

use crate::gates::bus16::testing::make_bus16;

pub enum Instruction {
    A(i32),
    C(Computation),
}

pub struct Computation {
    pub comp: (CompReg, Comp),
    pub dest: Dest,
    pub jump: Jump,
}

pub enum CompReg {
    A,
    M,
}

pub enum Comp {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    DPlus1,
    APlus1,
    DMinus1,
    AMinus1,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
}

pub enum Jump {
    None,
    Eq,
    NEq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Always,
}

#[derive(Eq, PartialEq)]
pub enum Dest {
    A,
    D,
    M,
    None,
}

impl Instruction {
    pub fn bus16(&self) -> Bus16 {
        let (p, n) = (Bit::Positive, Bit::Negative);
        let b = |bl: bool| if bl { p } else { n };
        match self {
            &Instruction::A(i) => make_bus16(i),
            Instruction::C(c) => {
                let a = match c.comp.0 {
                    CompReg::A => Bit::Negative,
                    CompReg::M => Bit::Positive,
                };
                let x: [Bit; 6] = match c.comp.1 {
                    Comp::Zero => [p, n, p, n, p, n],
                    Comp::One => [p, p, p, p, p, p],
                    Comp::MinusOne => [p, p, p, n, p, n],
                    Comp::D => [n, n, p, p, n, n],
                    Comp::A => [p, p, n, n, n, n],
                    Comp::NotD => [n, n, p, p, n, p],
                    Comp::NotA => [p, p, n, n, n, p],
                    Comp::MinusD => [n, n, p, p, p, p],
                    Comp::MinusA => [p, p, n, n, p, p],
                    Comp::DPlus1 => [n, p, p, p, p, p],
                    Comp::APlus1 => [p, p, n, p, p, p],
                    Comp::DMinus1 => [n, n, p, p, p, n],
                    Comp::AMinus1 => [p, p, n, p, p, p],
                    Comp::DPlusA => [n, n, n, n, p, n],
                    Comp::DMinusA => [n, p, n, n, p, p],
                    Comp::AMinusD => [n, n, n, p, p, p],
                    Comp::DAndA => [n, n, n, n, n, n],
                    Comp::DOrA => [n, p, n, p, n, p],
                };
                [
                    Bit::Positive,
                    Bit::Positive,
                    Bit::Positive,
                    a,
                    x[0],
                    x[1],
                    x[2],
                    x[3],
                    x[4],
                    x[5],
                    b(c.dest == Dest::A),
                    b(c.dest == Dest::D),
                    b(c.dest == Dest::M),
                    b(match c.jump {
                        Jump::Le | Jump::Lt | Jump::Ne | Jump::NEq | Jump::Always => true,
                        _ => false,
                    }),
                    b(match c.jump {
                        Jump::Eq | Jump::Le | Jump::Ge | Jump::Always => true,
                        _ => false,
                    }),
                    b(match c.jump {
                        Jump::Ge | Jump::Gt | Jump::Ne | Jump::NEq | Jump::Always => true,
                        _ => false,
                    }),
                ]
            }
        }
    }
}

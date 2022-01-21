use crate::instruction;

#[derive(Debug, Eq, PartialEq)]
pub enum AsmInst {
    A(A),
    C(instruction::Computation),
}

#[derive(Debug, Eq, PartialEq)]
pub enum A {
    Const(i32),
    Var(String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct C(instruction::Dest, Calc, instruction::Jump);

impl C {
    pub fn new(dest: instruction::Dest, calc: Calc, jump: instruction::Jump) -> Self {
        Self(dest, calc, jump)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Calc {
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
    M,
    NotM,
    MinusM,
    MPlus1,
    MMinus1,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Label(pub String);

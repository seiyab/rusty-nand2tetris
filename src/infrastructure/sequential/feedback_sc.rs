use std::marker::PhantomData;

use super::primitive::Dff;
use super::sequential_circuit::SequentialCircuit;
use crate::primitive::Bit;

pub struct FeedbackSC<T: FeedbackSCDef> {
    dff: Dff,
    p: PhantomData<T>,
}

impl<T: FeedbackSCDef> FeedbackSC<T> {
    pub fn new() -> Self {
        Self {
            dff: Dff::new(),
            p: PhantomData,
        }
    }
}

pub trait FeedbackSCDef {
    type Input;
    type Output;

    fn pre(i: &Self::Input, o: &Self::Output) -> Bit;
    fn post(b: Bit) -> Self::Output;
}

impl<T: FeedbackSCDef> SequentialCircuit for FeedbackSC<T> {
    type Input = T::Input;
    type Output = T::Output;

    fn tick(&self, input: &Self::Input) -> (Self::Output, Self) {
        let (buf_out, _) = self.dff.tick(&Bit::Negative);
        let o = T::post(buf_out);
        let buf_in = T::pre(input, &o);
        let (_, dff) = self.dff.tick(&buf_in);
        (
            o,
            Self {
                dff,
                p: PhantomData,
            },
        )
    }
}

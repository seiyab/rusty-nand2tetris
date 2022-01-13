use std::marker::PhantomData;

use crate::general::Zero;

use super::sequential_circuit::SequentialCircuit;

pub struct FeedbackSC<S: SequentialCircuit, T: FeedbackSCDef<S>> {
    sc: S,
    p: PhantomData<T>,
}

impl<S: SequentialCircuit + Zero, T: FeedbackSCDef<S>> Zero for FeedbackSC<S, T> {
    fn new() -> Self {
        Self {
            sc: S::new(),
            p: PhantomData,
        }
    }
}

pub trait FeedbackSCDef<S: SequentialCircuit> {
    type Input;
    type Output;
    type Feedback: Zero;

    fn pre(i: &Self::Input, f: &Self::Feedback) -> S::Input;
    fn post(i: &Self::Input, b: &S::Output) -> (Self::Output, Self::Feedback);
}

impl<S: SequentialCircuit, T: FeedbackSCDef<S>> SequentialCircuit for FeedbackSC<S, T> {
    type Input = T::Input;
    type Output = T::Output;

    fn tick(&self, input: &Self::Input) -> (Self::Output, Self) {
        let (buf_out, _) = self.sc.tick(&T::pre(input, &T::Feedback::new()));
        let (o, f) = T::post(&input, &buf_out);
        let buf_in = T::pre(input, &f);
        let (_, sc) = self.sc.tick(&buf_in);
        (o, Self { sc, p: PhantomData })
    }
}

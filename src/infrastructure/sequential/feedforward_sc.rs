use std::marker::PhantomData;

use crate::general::Zero;

use super::sequential_circuit::SequentialCircuit;

pub struct FeedforwardSC<S: SequentialCircuit, T: FeedforwardSCDef<S>> {
    sc: S,
    p: PhantomData<T>,
}

impl<S: SequentialCircuit, T: FeedforwardSCDef<S>> Zero for FeedforwardSC<S, T> {
    fn new() -> Self {
        Self {
            sc: T::new(),
            p: PhantomData,
        }
    }
}

pub trait FeedforwardSCDef<S: SequentialCircuit> {
    type Input;
    type Output;
    type Jump;

    fn new() -> S;
    fn pre(i: &Self::Input) -> (S::Input, Self::Jump);
    fn post(b: &S::Output, j: &Self::Jump) -> Self::Output;
}

impl<S: SequentialCircuit, T: FeedforwardSCDef<S>> SequentialCircuit for FeedforwardSC<S, T> {
    type Input = T::Input;
    type Output = T::Output;

    fn tick(&self, input: &Self::Input) -> (Self::Output, Self) {
        let (sc_in, jump) = T::pre(input);
        let (sc_out, sc) = self.sc.tick(&sc_in);
        let out = T::post(&sc_out, &jump);
        (out, Self { sc, p: PhantomData })
    }
}

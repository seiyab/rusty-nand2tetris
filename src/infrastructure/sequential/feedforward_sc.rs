use std::marker::PhantomData;

use super::sequential_circuit::SequentialCircuit;

pub struct FeedforwardSC<S: SequentialCircuit, T: FeedforwardSCDef<S>> {
    sc: S,
    p: PhantomData<T>,
}

impl<S: SequentialCircuit, T: FeedforwardSCDef<S>> FeedforwardSC<S, T> {
    pub fn new(sc: S) -> Self {
        Self { sc, p: PhantomData }
    }
}

pub trait FeedforwardSCDef<S: SequentialCircuit> {
    type Input;
    type Output;

    fn pre(i: &Self::Input) -> S::Input;
    fn post(b: &S::Output) -> Self::Output;
}

impl<S: SequentialCircuit, T: FeedforwardSCDef<S>> SequentialCircuit for FeedforwardSC<S, T> {
    type Input = T::Input;
    type Output = T::Output;

    fn tick(&self, input: &Self::Input) -> (Self::Output, Self) {
        let sc_in = T::pre(input);
        let (sc_out, sc) = self.sc.tick(&sc_in);
        let out = T::post(&sc_out);
        (out, Self { sc, p: PhantomData })
    }
}

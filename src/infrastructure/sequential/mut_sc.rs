use std::marker::PhantomData;

use crate::general::Zero;

use super::SequentialCircuit;

// Mutable Sequential Circuit
pub trait MutSC {
    type Input;
    type Output;

    fn tick(&mut self, input: &Self::Input) -> Self::Output;
}

pub struct FreeMutSC<T: SequentialCircuit>(T);

impl<T: SequentialCircuit + Zero> Zero for FreeMutSC<T> {
    fn new() -> Self {
        Self(T::new())
    }
}

impl<T: SequentialCircuit> MutSC for FreeMutSC<T> {
    type Input = T::Input;
    type Output = T::Output;

    fn tick(&mut self, input: &Self::Input) -> Self::Output {
        let (o, s) = self.0.tick(input);
        self.0 = s;
        o
    }
}

impl<T: MutSC> MutSC for Box<T> {
    type Input = T::Input;
    type Output = T::Output;

    fn tick(&mut self, input: &Self::Input) -> Self::Output {
        self.as_mut().tick(input)
    }
}

pub struct TupleMSC<A: MutSC, B: MutSC>(A, B);

impl<A: MutSC, B: MutSC> MutSC for TupleMSC<A, B> {
    type Input = (A::Input, B::Input);
    type Output = (A::Output, B::Output);

    fn tick(&mut self, input: &Self::Input) -> Self::Output {
        (self.0.tick(&input.0), self.1.tick(&input.1))
    }
}

impl<A: MutSC + Zero, B: MutSC + Zero> Zero for TupleMSC<A, B> {
    fn new() -> Self {
        Self(A::new(), B::new())
    }
}

pub struct FeedforwardMSC<A: MutSC, D: FeedforwardMSCDef<A>> {
    msc: A,
    p: PhantomData<D>,
}

pub trait FeedforwardMSCDef<S: MutSC> {
    type Input;
    type Output;
    type Jump;

    fn pre(i: &Self::Input) -> (S::Input, Self::Jump);
    fn post(b: &S::Output, j: &Self::Jump) -> Self::Output;
}

impl<A: MutSC, D: FeedforwardMSCDef<A>> MutSC for FeedforwardMSC<A, D> {
    type Input = D::Input;
    type Output = D::Output;

    fn tick(&mut self, input: &Self::Input) -> Self::Output {
        let (msc_in, jump) = D::pre(input);
        let msc_out = self.msc.tick(&msc_in);
        D::post(&msc_out, &jump)
    }
}

impl<A: MutSC + Zero, D: FeedforwardMSCDef<A>> Zero for FeedforwardMSC<A, D> {
    fn new() -> Self {
        Self {
            msc: A::new(),
            p: PhantomData,
        }
    }
}

use super::SequentialCircuit;

// Mutable Sequential Circuit
pub trait MutSC {
    type Input;
    type Output;

    fn tick(&mut self, input: &Self::Input) -> Self::Output;
}

pub struct FreeMutSC<T: SequentialCircuit>(T);

impl<T: SequentialCircuit> FreeMutSC<T> {
    pub fn new(t: T) -> Self {
        Self(t)
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

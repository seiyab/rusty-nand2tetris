pub trait SequentialCircuit {
    type Input;
    type Output;

    fn tick(&self, input: &Self::Input) -> (Self::Output, Self);
}

impl<T: SequentialCircuit> SequentialCircuit for Box<T> {
    type Input = T::Input;
    type Output = T::Output;

    fn tick(&self, input: &Self::Input) -> (Self::Output, Self) {
        let (o, s) = self.as_ref().tick(input);
        (o, Box::new(s))
    }
}

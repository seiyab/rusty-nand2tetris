pub trait SequentialCircuit {
    type Input;
    type Output;

    fn tick(&self, input: &Self::Input) -> (Self::Output, Self);
}

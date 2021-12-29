use std::marker::PhantomData;

use super::sequential_circuit::SequentialCircuit;

pub struct FuncSC<'a, State, Input, Output, F>
where
    F: Fn(&State, Input) -> (Output, State),
{
    state: State,
    f: &'a F,
    p: PhantomData<(Input, Output)>,
}

impl<'a, S, I, O, F: Fn(&S, I) -> (O, S)> FuncSC<'a, S, I, O, F> {
    pub fn of(state: S, transition: &'a F) -> Self {
        FuncSC {
            state,
            f: transition,
            p: PhantomData,
        }
    }

    fn transition(&self, input: I) -> (O, S) {
        let f = self.f;
        f(&self.state, input)
    }
}

impl<'a, S, I, O, F: Fn(&S, I) -> (O, S)> SequentialCircuit for FuncSC<'a, S, I, O, F> {
    type Input = I;
    type Output = O;

    fn tick(&self, input: Self::Input) -> (Self::Output, Self) {
        let (output, state) = self.transition(input);
        (
            output,
            FuncSC {
                state,
                f: self.f,
                p: PhantomData,
            },
        )
    }
}

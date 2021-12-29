use crate::infrastructure::sequential::FuncSC;
use crate::infrastructure::sequential::SequentialCircuit;
use crate::primitive::Bit;

pub struct Dff(FuncSC<'static, Bit, Bit, Bit, fn(&Bit, Bit) -> (Bit, Bit)>);

impl Dff {
    fn new() -> Self {
        Self(FuncSC::of(
            Bit::Negative,
            &(dff_fn as fn(&Bit, Bit) -> (Bit, Bit)),
        ))
    }
}

impl SequentialCircuit for Dff {
    type Input = Bit;
    type Output = Bit;

    fn tick(&self, i: Self::Input) -> (Self::Output, Self) {
        let Self(f) = self;
        let (o, nx) = f.tick(i);
        (o, Self(nx))
    }
}

fn dff_fn(s: &Bit, i: Bit) -> (Bit, Bit) {
    (*s, i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_bit_equals;
    use crate::infrastructure::sequential::SequentialCircuit;

    #[test]
    fn dff_works() {
        let dff = Dff::new();
        let (o, dff) = dff.tick(Bit::Positive);
        assert_bit_equals!(o, Bit::Negative);
        let (o, dff) = dff.tick(Bit::Negative);
        assert_bit_equals!(o, Bit::Positive);
        let (o, dff) = dff.tick(Bit::Negative);
        assert_bit_equals!(o, Bit::Negative);
        let (o, dff) = dff.tick(Bit::Negative);
        assert_bit_equals!(o, Bit::Negative);
        let (o, dff) = dff.tick(Bit::Positive);
        assert_bit_equals!(o, Bit::Negative);
        let (o, _) = dff.tick(Bit::Positive);
        assert_bit_equals!(o, Bit::Positive);
    }
}

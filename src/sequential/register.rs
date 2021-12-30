use crate::gates::bit;
use crate::infrastructure::sequential::{FeedbackSC, FeedbackSCDef};
use crate::primitive::Bit;

struct RegisterImpl;

pub type Register = FeedbackSC<RegisterImpl>;

pub struct RegisterInput {
    input: Bit,
    load: Bit,
}

impl FeedbackSCDef for RegisterImpl {
    type Input = RegisterInput;
    type Output = Bit;

    fn pre(inputs: &Self::Input, output: &Self::Output) -> Bit {
        let RegisterInput { input, load } = *inputs;
        bit::mux(*output, input, load)
    }
    fn post(buf: Bit) -> Self::Output {
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::assert_bit_equals;
    use crate::infrastructure::sequential::SequentialCircuit;

    #[test]
    fn register_works() {
        let r = Register::new();
        let (p, n) = (Bit::Positive, Bit::Negative);
        let (o, r) = r.tick(&RegisterInput { input: p, load: n });
        assert_bit_equals!(o, n);
        let (o, r) = r.tick(&RegisterInput { input: p, load: n });
        assert_bit_equals!(o, n);
        let (o, r) = r.tick(&RegisterInput { input: p, load: n });
        assert_bit_equals!(o, n);
        let (o, r) = r.tick(&RegisterInput { input: p, load: p });
        assert_bit_equals!(o, n);
        let (o, r) = r.tick(&RegisterInput { input: n, load: n });
        assert_bit_equals!(o, p);
        let (o, r) = r.tick(&RegisterInput { input: n, load: p });
        assert_bit_equals!(o, p);
        let (o, _) = r.tick(&RegisterInput { input: p, load: p });
        assert_bit_equals!(o, n);
    }
}

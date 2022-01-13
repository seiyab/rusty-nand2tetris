use crate::gates::bit;
use crate::gates::bus16;
use crate::general::Zero;
use crate::infrastructure::sequential::primitive::Dff;
use crate::infrastructure::sequential::{
    ArraySC16, FeedbackSC, FeedbackSCDef, FeedforwardSC, FeedforwardSCDef,
};
use crate::primitive::Bit;

pub struct RegisterImpl;

pub type Register = FeedbackSC<Dff, RegisterImpl>;

pub struct RegisterInput {
    pub input: Bit,
    pub load: Bit,
}

impl FeedbackSCDef<Dff> for RegisterImpl {
    type Input = RegisterInput;
    type Output = Bit;
    type Feedback = Bit;

    fn pre(inputs: &Self::Input, feedback: &Self::Feedback) -> Bit {
        let RegisterInput { input, load } = *inputs;
        bit::mux(*feedback, input, load)
    }
    fn post(_: &Self::Input, buf: &Bit) -> (Self::Output, Self::Feedback) {
        (*buf, *buf)
    }
}

pub struct Register16Impl;

pub type Register16 = FeedforwardSC<ArraySC16<Register>, Register16Impl>;

pub struct Register16Input {
    pub input: bus16::Bus16,
    pub load: Bit,
}

impl FeedforwardSCDef<ArraySC16<Register>> for Register16Impl {
    type Input = Register16Input;
    type Output = bus16::Bus16;
    type Jump = ();

    fn new() -> ArraySC16<Register> {
        ArraySC16::new()
    }
    fn pre(i: &Self::Input) -> ([RegisterInput; 16], ()) {
        let Self::Input { input, load } = i;
        let b = input;
        let r = |x: Bit, l: Bit| RegisterInput { input: x, load: l };
        let l = *load;
        (
            [
                r(b[0x0], l),
                r(b[0x1], l),
                r(b[0x2], l),
                r(b[0x3], l),
                r(b[0x4], l),
                r(b[0x5], l),
                r(b[0x6], l),
                r(b[0x7], l),
                r(b[0x8], l),
                r(b[0x9], l),
                r(b[0xa], l),
                r(b[0xb], l),
                r(b[0xc], l),
                r(b[0xd], l),
                r(b[0xe], l),
                r(b[0xf], l),
            ],
            (),
        )
    }
    fn post(b: &[Bit; 16], _: &()) -> Self::Output {
        b.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::assert_bit_equals;
    use crate::assert_bus16_equals;
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

    #[test]
    fn register16_works() {
        let (p, n) = (Bit::Positive, Bit::Negative);
        let fxt1 = [p; 16];
        let fxt2 = [n; 16];
        let fxt3 = [p, p, n, n, p, p, p, p, n, n, n, n, p, n, p, n];
        let fxt4 = [n, p, p, n, p, p, n, n, p, n, p, n, p, p, n, n];
        let r = Register16::new();
        let (o, r) = r.tick(&Register16Input {
            input: fxt1.clone(),
            load: Bit::Positive,
        });
        assert_bus16_equals!(&fxt2, &o);
        let (o, r) = r.tick(&Register16Input {
            input: fxt2.clone(),
            load: Bit::Negative,
        });
        assert_bus16_equals!(&fxt1, &o);
        let (o, r) = r.tick(&Register16Input {
            input: fxt2.clone(),
            load: Bit::Positive,
        });
        assert_bus16_equals!(&fxt1, &o);
        let (o, r) = r.tick(&Register16Input {
            input: fxt3.clone(),
            load: Bit::Positive,
        });
        assert_bus16_equals!(&fxt2, &o);
        let (o, r) = r.tick(&Register16Input {
            input: fxt4.clone(),
            load: Bit::Positive,
        });
        assert_bus16_equals!(&fxt3, &o);
        let (o, _) = r.tick(&Register16Input {
            input: fxt1,
            load: Bit::Positive,
        });
        assert_bus16_equals!(&fxt4, &o);
    }
}

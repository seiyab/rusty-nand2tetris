use crate::gates::bit;
use crate::gates::bus16;
use crate::general::T16;
use crate::infrastructure::sequential::primitive::Dff;
use crate::infrastructure::sequential::{
    FeedbackSC, FeedbackSCDef, FeedforwardSC, FeedforwardSCDef, TupleSC2,
};
use crate::primitive::Bit;

pub struct RegisterImpl;

pub type Register = FeedbackSC<RegisterImpl>;

pub struct RegisterInput {
    pub input: Bit,
    pub load: Bit,
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

pub struct Register16Impl;

type R2 = TupleSC2<Register, Register>;
type R4 = TupleSC2<R2, R2>;
type R8 = TupleSC2<R4, R4>;
type R16 = TupleSC2<R8, R8>;

pub type Register16 = FeedforwardSC<R16, Register16Impl>;

pub struct Register16Input {
    pub input: bus16::Bus16,
    pub load: Bit,
}

impl FeedforwardSCDef<R16> for Register16Impl {
    type Input = Register16Input;
    type Output = bus16::Bus16;
    type Jump = ();

    fn new() -> R16 {
        TupleSC2::new(
            TupleSC2::new(
                TupleSC2::new(
                    TupleSC2::new(Register::new(), Register::new()),
                    TupleSC2::new(Register::new(), Register::new()),
                ),
                TupleSC2::new(
                    TupleSC2::new(Register::new(), Register::new()),
                    TupleSC2::new(Register::new(), Register::new()),
                ),
            ),
            TupleSC2::new(
                TupleSC2::new(
                    TupleSC2::new(Register::new(), Register::new()),
                    TupleSC2::new(Register::new(), Register::new()),
                ),
                TupleSC2::new(
                    TupleSC2::new(Register::new(), Register::new()),
                    TupleSC2::new(Register::new(), Register::new()),
                ),
            ),
        )
    }
    fn pre(i: &Self::Input) -> (T16<RegisterInput>, ()) {
        let Self::Input { input, load } = i;
        let bus16::Bus16(b) = input;
        let r = |x: Bit, l: Bit| RegisterInput { input: x, load: l };
        let l = *load;
        (
            (
                (
                    ((r(b[0], l), r(b[1], l)), (r(b[2], l), r(b[3], l))),
                    ((r(b[4], l), r(b[5], l)), (r(b[6], l), r(b[7], l))),
                ),
                (
                    ((r(b[8], l), r(b[9], l)), (r(b[10], l), r(b[11], l))),
                    ((r(b[12], l), r(b[13], l)), (r(b[14], l), r(b[15], l))),
                ),
            ),
            (),
        )
    }
    fn post(b: &T16<Bit>, j: &()) -> Self::Output {
        let (
            (((o0, o1), (o2, o3)), ((o4, o5), (o6, o7))),
            (((o8, o9), (o10, o11)), ((o12, o13), (o14, o15))),
        ) = b;
        bus16::Bus16([
            *o0, *o1, *o2, *o3, *o4, *o5, *o6, *o7, *o8, *o9, *o10, *o11, *o12, *o13, *o14, *o15,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::assert_bit_equals;
    use crate::assert_bus16_equals;
    use crate::gates::bus16::Bus16;
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
        let fxt1 = Bus16([p; 16]);
        let fxt2 = Bus16([n; 16]);
        let fxt3 = Bus16([p, p, n, n, p, p, p, p, n, n, n, n, p, n, p, n]);
        let fxt4 = Bus16([n, p, p, n, p, p, n, n, p, n, p, n, p, p, n, n]);
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

pub mod dff {
    use crate::infrastructure::sequential::FuncSC;
    use crate::primitive::Bit;

    pub type DFF = FuncSC<'static, Bit, Bit, Bit, fn(&Bit, Bit) -> (Bit, Bit)>;

    pub fn of(i: Bit) -> DFF {
        FuncSC::of(i, &d)
    }

    fn dff_fn(s: &Bit, i: Bit) -> (Bit, Bit) {
        (*s, i)
    }

    const d: fn(&Bit, Bit) -> (Bit, Bit) = dff_fn;

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::assert_bit_equals;
        use crate::infrastructure::sequential::SequentialCircuit;

        #[test]
        fn dff_works() {
            let dff = of(Bit::Negative);
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
}

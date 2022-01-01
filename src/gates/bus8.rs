use crate::primitive::Bit;

use super::bit;

pub type Bus8 = [Bit; 8];

#[macro_export]
macro_rules! assert_bus8_equals {
    ($actual:expr, $expected:expr) => {
        let a = $actual;
        let b = $expected;
        for i in 0..8 {
            assert_bit_equals!(a[i], b[i]);
        }
    };
}

pub fn or8way(x: Bus8) -> Bit {
    bit::or(
        bit::or(bit::or(x[0], x[1]), bit::or(x[2], x[3])),
        bit::or(bit::or(x[4], x[5]), bit::or(x[6], x[7])),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_bit_equals;
    use crate::assert_bus8_equals;
    use crate::primitive::Bit;

    #[test]
    fn assert_bus8_equals_works() {
        assert_bus8_equals!([Bit::Positive; 8], [Bit::Positive; 8]);
        assert_bus8_equals!([Bit::Negative; 8], [Bit::Negative; 8]);
    }

    #[test]
    fn or8way_works() {
        assert_bit_equals!(or8way([Bit::Positive; 8]), Bit::Positive);
        assert_bit_equals!(or8way([Bit::Negative; 8]), Bit::Negative);
        assert_bit_equals!(
            or8way([
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
            ]),
            Bit::Positive
        );
        assert_bit_equals!(
            or8way([
                Bit::Negative,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
            ]),
            Bit::Positive
        );
        assert_bit_equals!(
            or8way([
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
            ]),
            Bit::Positive
        );
    }
}

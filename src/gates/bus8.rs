use crate::primitive::Bit;

use super::bit;

pub struct Bus8(pub [Bit; 8]);

#[macro_export]
macro_rules! assert_bus8_equals {
    ($actual:expr, $expected:expr) => {
        let Bus8(a) = $actual;
        let Bus8(b) = $expected;
        for i in 0..8 {
            assert_bit_equals!(a[i], b[i]);
        }
    };
}

pub fn or8way(x: Bus8) -> Bit {
    bit::or(
        bit::or(bit::or(x.0[0], x.0[1]), bit::or(x.0[2], x.0[3])),
        bit::or(bit::or(x.0[4], x.0[5]), bit::or(x.0[6], x.0[7])),
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
        assert_bus8_equals!(Bus8([Bit::Positive; 8]), Bus8([Bit::Positive; 8]));
        assert_bus8_equals!(Bus8([Bit::Negative; 8]), Bus8([Bit::Negative; 8]));
    }

    #[test]
    fn or8way_works() {
        assert_bit_equals!(or8way(Bus8([Bit::Positive; 8])), Bit::Positive);
        assert_bit_equals!(or8way(Bus8([Bit::Negative; 8])), Bit::Negative);
        assert_bit_equals!(
            or8way(Bus8([
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
            ])),
            Bit::Positive
        );
        assert_bit_equals!(
            or8way(Bus8([
                Bit::Negative,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
            ])),
            Bit::Positive
        );
        assert_bit_equals!(
            or8way(Bus8([
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
            ])),
            Bit::Positive
        );
    }
}

use crate::primitive::Bus16;

use super::bit;

pub fn not(x: Bus16) -> Bus16 {
    Bus16([
        bit::not(x.0[0]),
        bit::not(x.0[1]),
        bit::not(x.0[2]),
        bit::not(x.0[3]),
        bit::not(x.0[4]),
        bit::not(x.0[5]),
        bit::not(x.0[6]),
        bit::not(x.0[7]),
        bit::not(x.0[8]),
        bit::not(x.0[9]),
        bit::not(x.0[10]),
        bit::not(x.0[11]),
        bit::not(x.0[12]),
        bit::not(x.0[13]),
        bit::not(x.0[14]),
        bit::not(x.0[15]),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_bit_equals;
    use crate::assert_bus16_equals;
    use crate::primitive::Bit;

    #[test]
    fn not_works() {
        assert_bus16_equals!(not(Bus16([Bit::Positive; 16])), Bus16([Bit::Negative; 16]));
        assert_bus16_equals!(not(Bus16([Bit::Negative; 16])), Bus16([Bit::Positive; 16]));
        assert_bus16_equals!(
            not(Bus16([
                Bit::Negative,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Positive,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Positive,
            ])),
            Bus16([
                Bit::Positive,
                Bit::Negative,
                Bit::Positive,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Positive,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
            ])
        );
    }
}

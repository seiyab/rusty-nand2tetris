use crate::primitive::Bus16;

use super::bit;

pub fn not(x: &Bus16) -> Bus16 {
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

pub fn and(x: &Bus16, y: &Bus16) -> Bus16 {
    Bus16([
        bit::and(x.0[0], y.0[0]),
        bit::and(x.0[1], y.0[1]),
        bit::and(x.0[2], y.0[2]),
        bit::and(x.0[3], y.0[3]),
        bit::and(x.0[4], y.0[4]),
        bit::and(x.0[5], y.0[5]),
        bit::and(x.0[6], y.0[6]),
        bit::and(x.0[7], y.0[7]),
        bit::and(x.0[8], y.0[8]),
        bit::and(x.0[9], y.0[9]),
        bit::and(x.0[10], y.0[10]),
        bit::and(x.0[11], y.0[11]),
        bit::and(x.0[12], y.0[12]),
        bit::and(x.0[13], y.0[13]),
        bit::and(x.0[14], y.0[14]),
        bit::and(x.0[15], y.0[15]),
    ])
}

pub fn or(x: &Bus16, y: &Bus16) -> Bus16 {
    Bus16([
        bit::or(x.0[0], y.0[0]),
        bit::or(x.0[1], y.0[1]),
        bit::or(x.0[2], y.0[2]),
        bit::or(x.0[3], y.0[3]),
        bit::or(x.0[4], y.0[4]),
        bit::or(x.0[5], y.0[5]),
        bit::or(x.0[6], y.0[6]),
        bit::or(x.0[7], y.0[7]),
        bit::or(x.0[8], y.0[8]),
        bit::or(x.0[9], y.0[9]),
        bit::or(x.0[10], y.0[10]),
        bit::or(x.0[11], y.0[11]),
        bit::or(x.0[12], y.0[12]),
        bit::or(x.0[13], y.0[13]),
        bit::or(x.0[14], y.0[14]),
        bit::or(x.0[15], y.0[15]),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_bit_equals;
    use crate::assert_bus16_equals;
    use crate::primitive::Bit;

    const FXT: Bus16 = Bus16([
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
    ]);

    #[test]
    fn not_works() {
        assert_bus16_equals!(not(&Bus16([Bit::Positive; 16])), Bus16([Bit::Negative; 16]));
        assert_bus16_equals!(not(&Bus16([Bit::Negative; 16])), Bus16([Bit::Positive; 16]));
        assert_bus16_equals!(
            not(&FXT),
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

    #[test]
    fn and_works() {
        assert_bus16_equals!(and(&FXT, &Bus16([Bit::Positive; 16])), &FXT);
        assert_bus16_equals!(
            and(&FXT, &Bus16([Bit::Negative; 16])),
            Bus16([Bit::Negative; 16])
        );
    }

    #[test]
    fn or_works() {
        assert_bus16_equals!(
            or(&FXT, &Bus16([Bit::Positive; 16])),
            Bus16([Bit::Positive; 16])
        );
        assert_bus16_equals!(or(&FXT, &Bus16([Bit::Negative; 16])), &FXT);
    }
}

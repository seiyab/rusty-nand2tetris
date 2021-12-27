#[derive(Clone, Copy)]
pub enum Bit {
    Positive,
    Negative,
}

impl Bit {
    pub fn nand(&self, another: Bit) -> Bit {
        match self {
            Self::Positive => match another {
                Self::Positive => Self::Negative,
                Self::Negative => Self::Positive,
            },
            Self::Negative => Self::Positive,
        }
    }
}

#[macro_export]
macro_rules! assert_bit_equals {
    ($actual:expr, $expected:expr) => {
        let a = match $actual {
            Bit::Positive => true,
            Bit::Negative => false,
        };
        let e = match $expected {
            Bit::Positive => true,
            Bit::Negative => false,
        };
        assert!(a == e, stringify!(($actual, $expected)));
    };
}

pub struct Bus16(pub [Bit; 16]);

#[macro_export]
macro_rules! assert_bus16_equals {
    ($actual:expr, $expected:expr) => {
        let Bus16(a) = $actual;
        let Bus16(b) = $expected;
        for i in 0..16 {
            assert_bit_equals!(a[i], b[i]);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nand_works() {
        assert_bit_equals!(Bit::Positive.nand(Bit::Positive), Bit::Negative);
        assert_bit_equals!(Bit::Positive.nand(Bit::Negative), Bit::Positive);
        assert_bit_equals!(Bit::Negative.nand(Bit::Positive), Bit::Positive);
        assert_bit_equals!(Bit::Negative.nand(Bit::Negative), Bit::Positive);
    }

    #[test]
    fn assert_bus16_equals_works() {
        assert_bus16_equals!(Bus16([Bit::Positive; 16]), Bus16([Bit::Positive; 16]));
    }
}

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
macro_rules! assert_pin_equals {
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

#[cfg(test)]
mod tests {
    use super::Bit;

    #[test]
    fn nand_works() {
        assert_pin_equals!(Bit::Positive.nand(Bit::Positive), Bit::Negative);
        assert_pin_equals!(Bit::Positive.nand(Bit::Negative), Bit::Positive);
        assert_pin_equals!(Bit::Negative.nand(Bit::Positive), Bit::Positive);
        assert_pin_equals!(Bit::Negative.nand(Bit::Negative), Bit::Positive);
    }
}

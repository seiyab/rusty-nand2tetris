#[derive(Clone, Copy)]
pub enum Pin {
    Positive,
    Negative,
}

impl Pin {
    pub fn nand(&self, another: Pin) -> Pin {
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
            Pin::Positive => true,
            Pin::Negative => false,
        };
        let e = match $expected {
            Pin::Positive => true,
            Pin::Negative => false,
        };
        assert!(a == e, stringify!(($actual, $expected)));
    };
}

#[cfg(test)]
mod tests {
    use super::Pin;

    #[test]
    fn nand_works() {
        assert_pin_equals!(Pin::Positive.nand(Pin::Positive), Pin::Negative);
        assert_pin_equals!(Pin::Positive.nand(Pin::Negative), Pin::Positive);
        assert_pin_equals!(Pin::Negative.nand(Pin::Positive), Pin::Positive);
        assert_pin_equals!(Pin::Negative.nand(Pin::Negative), Pin::Positive);
    }
}

use crate::primitive::Bit;

pub type Bus3 = [Bit; 3];

pub mod testing {
    use super::*;
    use crate::gates::bit::testing::make_bit;

    pub fn make_bus3(i: i32) -> Bus3 {
        let mut b3 = [Bit::Negative; 3];
        for b in 0..3 {
            b3[b] = make_bit(i & (1 << (2 - b)) != 0);
        }
        b3
    }

    #[cfg(testing)]
    mod tests {
        use super::*;
        use crate::assert_bit_equals;

        #[test]
        fn make_bus3_works() {
            let a = make_bus3(3);
            assert_bit_equals!(a[0], Bit::Negative);
            assert_bit_equals!(a[1], Bit::Positive);
            assert_bit_equals!(a[2], Bit::Positive);

            let a = make_bus3(5);
            assert_bit_equals!(a[0], Bit::Positive);
            assert_bit_equals!(a[1], Bit::Negative);
            assert_bit_equals!(a[2], Bit::Positive);

            let a = make_bus3(6);
            assert_bit_equals!(a[0], Bit::Positive);
            assert_bit_equals!(a[1], Bit::Positive);
            assert_bit_equals!(a[2], Bit::Negative);
        }
    }
}

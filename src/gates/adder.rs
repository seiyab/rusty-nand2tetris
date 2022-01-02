use crate::gates::bit;
use crate::gates::bus16;
use crate::primitive::Bit;

struct AdderOut {
    sum: Bit,
    carry: Bit,
}

fn half_adder(a: Bit, b: Bit) -> AdderOut {
    AdderOut {
        sum: bit::xor(a, b),
        carry: bit::and(a, b),
    }
}

fn full_adder(a: Bit, b: Bit, c: Bit) -> AdderOut {
    AdderOut {
        sum: bit::mux(bit::xor(a, b), bit::not(bit::xor(a, b)), c),
        carry: bit::mux(bit::and(a, b), bit::or(a, b), c),
    }
}

pub fn add16(a: &bus16::Bus16, b: &bus16::Bus16) -> bus16::Bus16 {
    let AdderOut { sum: x15, carry } = half_adder(a[15], b[15]);
    let AdderOut { sum: x14, carry } = full_adder(a[14], b[14], carry);
    let AdderOut { sum: x13, carry } = full_adder(a[13], b[13], carry);
    let AdderOut { sum: x12, carry } = full_adder(a[12], b[12], carry);
    let AdderOut { sum: x11, carry } = full_adder(a[11], b[11], carry);
    let AdderOut { sum: x10, carry } = full_adder(a[10], b[10], carry);
    let AdderOut { sum: x9, carry } = full_adder(a[9], b[9], carry);
    let AdderOut { sum: x8, carry } = full_adder(a[8], b[8], carry);
    let AdderOut { sum: x7, carry } = full_adder(a[7], b[7], carry);
    let AdderOut { sum: x6, carry } = full_adder(a[6], b[6], carry);
    let AdderOut { sum: x5, carry } = full_adder(a[5], b[5], carry);
    let AdderOut { sum: x4, carry } = full_adder(a[4], b[4], carry);
    let AdderOut { sum: x3, carry } = full_adder(a[3], b[3], carry);
    let AdderOut { sum: x2, carry } = full_adder(a[2], b[2], carry);
    let AdderOut { sum: x1, carry } = full_adder(a[1], b[1], carry);
    let AdderOut { sum: x0, .. } = full_adder(a[0], b[0], carry);
    [
        x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15,
    ]
}

pub fn inc16(a: &bus16::Bus16) -> bus16::Bus16 {
    add16(
        a,
        &[
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Negative,
            Bit::Positive,
        ],
    )
}

pub struct AluOut {
    out: bus16::Bus16,
    zr: Bit,
    ng: Bit,
}

pub struct AluControl {
    zx: Bit,
    nx: Bit,
    zy: Bit,
    ny: Bit,
    f: Bit,
    no: Bit,
}

pub fn alu(x: &bus16::Bus16, y: &bus16::Bus16, ctrl: AluControl) -> AluOut {
    let x = bus16::mux(&x, &[Bit::Negative; 16], ctrl.zx);
    let x = bus16::mux(&x, &bus16::not(&x), ctrl.nx);
    let y = bus16::mux(&y, &[Bit::Negative; 16], ctrl.zy);
    let y = bus16::mux(&y, &bus16::not(&y), ctrl.ny);

    let out = bus16::mux(&bus16::and(&x, &y), &add16(&x, &y), ctrl.f);
    let out = bus16::mux(&out, &bus16::not(&out), ctrl.no);

    let zr = bit::not(bus16::or16way(&out));
    let ng = out[0];
    AluOut { out, zr, ng }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_bit_equals;
    use crate::assert_bus16_equals;
    use crate::gates::bus16::testing::*;
    use std::ops::Not;

    #[test]
    fn add16_works() {
        let fixtures = [0, 1, 2, 3, 100, 1000, 10000, -1, -10];
        for &x in &fixtures {
            for &y in &fixtures {
                let x16 = make_bus16(x);
                let y16 = make_bus16(y);
                let z16 = make_bus16(x + y);
                assert_bus16_equals!(add16(&x16, &y16), &z16);
            }
        }
    }

    #[test]
    fn inc16_works() {
        let fixtures = [0, 1, 2, 3, 100, 1000, 10000, -1, -10];
        for &x in &fixtures {
            for &y in &fixtures {
                let x16 = make_bus16(x);
                let y16 = make_bus16(1);
                let z16 = make_bus16(x + 1);
                assert_bus16_equals!(add16(&x16, &y16), &z16);
            }
        }
    }

    #[test]
    fn make_bus16_works() {
        assert_bus16_equals!(make_bus16(0), [Bit::Negative; 16]);
        assert_bus16_equals!(
            make_bus16(1),
            [
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
            ]
        );
        assert_bus16_equals!(make_bus16(-1), [Bit::Positive; 16]);
        assert_bus16_equals!(
            make_bus16(100),
            [
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
                Bit::Positive,
                Bit::Negative,
                Bit::Negative,
            ]
        );
    }

    #[test]
    fn alu_works() {
        let fixtures = [0, 1, 2, 10000, -1, -10];
        for &x in &fixtures {
            for &y in &fixtures {
                let x16 = make_bus16(x);
                let y16 = make_bus16(y);

                let cases: [([i32; 6], i32); 18] = [
                    ([1, 0, 1, 0, 1, 0], 0),
                    ([1, 1, 1, 1, 1, 1], 1),
                    ([1, 1, 1, 0, 1, 0], -1),
                    ([0, 0, 1, 1, 0, 0], x),
                    ([1, 1, 0, 0, 0, 0], y),
                    ([0, 0, 1, 1, 0, 1], x.not()),
                    ([1, 1, 0, 0, 0, 1], y.not()),
                    ([0, 0, 1, 1, 1, 1], -x),
                    ([1, 1, 0, 0, 1, 1], -y),
                    ([0, 1, 1, 1, 1, 1], x + 1),
                    ([1, 1, 0, 1, 1, 1], y + 1),
                    ([0, 0, 1, 1, 1, 0], x - 1),
                    ([1, 1, 0, 0, 1, 0], y - 1),
                    ([0, 0, 0, 0, 1, 0], x + y),
                    ([0, 1, 0, 0, 1, 1], x - y),
                    ([0, 0, 0, 1, 1, 1], y - x),
                    ([0, 0, 0, 0, 0, 0], x & y),
                    ([0, 1, 0, 1, 0, 1], x | y),
                ];
                for &([zx, nx, zy, ny, f, no], expected) in cases.iter() {
                    let b = |i: i32| if i == 1 { Bit::Positive } else { Bit::Negative };
                    let AluOut { out, .. } = alu(
                        &x16,
                        &y16,
                        AluControl {
                            zx: b(zx),
                            nx: b(nx),
                            zy: b(zy),
                            ny: b(ny),
                            f: b(f),
                            no: b(no),
                        },
                    );
                    assert_bus16_equals!(&out, &make_bus16(expected));
                }
            }
        }
    }
}

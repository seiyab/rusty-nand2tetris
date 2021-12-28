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
    let AdderOut { sum: x15, carry } = half_adder(a.0[15], b.0[15]);
    let AdderOut { sum: x14, carry } = full_adder(a.0[14], b.0[14], carry);
    let AdderOut { sum: x13, carry } = full_adder(a.0[13], b.0[13], carry);
    let AdderOut { sum: x12, carry } = full_adder(a.0[12], b.0[12], carry);
    let AdderOut { sum: x11, carry } = full_adder(a.0[11], b.0[11], carry);
    let AdderOut { sum: x10, carry } = full_adder(a.0[10], b.0[10], carry);
    let AdderOut { sum: x9, carry } = full_adder(a.0[9], b.0[9], carry);
    let AdderOut { sum: x8, carry } = full_adder(a.0[8], b.0[8], carry);
    let AdderOut { sum: x7, carry } = full_adder(a.0[7], b.0[7], carry);
    let AdderOut { sum: x6, carry } = full_adder(a.0[6], b.0[6], carry);
    let AdderOut { sum: x5, carry } = full_adder(a.0[5], b.0[5], carry);
    let AdderOut { sum: x4, carry } = full_adder(a.0[4], b.0[4], carry);
    let AdderOut { sum: x3, carry } = full_adder(a.0[3], b.0[3], carry);
    let AdderOut { sum: x2, carry } = full_adder(a.0[2], b.0[2], carry);
    let AdderOut { sum: x1, carry } = full_adder(a.0[1], b.0[1], carry);
    let AdderOut { sum: x0, .. } = full_adder(a.0[0], b.0[0], carry);
    bus16::Bus16([
        x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15,
    ])
}

pub fn inc16(a: &bus16::Bus16) -> bus16::Bus16 {
    add16(
        a,
        &bus16::Bus16([
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
        ]),
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
    let x = bus16::mux(&x, &bus16::Bus16([Bit::Negative; 16]), ctrl.zx);
    let x = bus16::mux(&x, &bus16::not(&x), ctrl.nx);
    let y = bus16::mux(&y, &bus16::Bus16([Bit::Negative; 16]), ctrl.zy);
    let y = bus16::mux(&y, &bus16::not(&y), ctrl.ny);

    let out = bus16::mux(&bus16::and(&x, &y), &add16(&x, &y), ctrl.f);
    let out = bus16::mux(&out, &bus16::not(&out), ctrl.no);

    let zr = bit::not(bus16::or16way(&out));
    let ng = out.0[0];
    AluOut { out, zr, ng }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_bit_equals;
    use crate::assert_bus16_equals;
    use crate::gates::bus16::Bus16;
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
        assert_bus16_equals!(make_bus16(0), bus16::Bus16([Bit::Negative; 16]));
        assert_bus16_equals!(
            make_bus16(1),
            bus16::Bus16([
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
            ])
        );
        assert_bus16_equals!(make_bus16(-1), bus16::Bus16([Bit::Positive; 16]));
        assert_bus16_equals!(
            make_bus16(100),
            bus16::Bus16([
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
            ])
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

    fn make_bus16(i: i32) -> bus16::Bus16 {
        bus16::Bus16([
            make_bit(i & (1 << 15) != 0),
            make_bit(i & (1 << 14) != 0),
            make_bit(i & (1 << 13) != 0),
            make_bit(i & (1 << 12) != 0),
            make_bit(i & (1 << 11) != 0),
            make_bit(i & (1 << 10) != 0),
            make_bit(i & (1 << 9) != 0),
            make_bit(i & (1 << 8) != 0),
            make_bit(i & (1 << 7) != 0),
            make_bit(i & (1 << 6) != 0),
            make_bit(i & (1 << 5) != 0),
            make_bit(i & (1 << 4) != 0),
            make_bit(i & (1 << 3) != 0),
            make_bit(i & (1 << 2) != 0),
            make_bit(i & (1 << 1) != 0),
            make_bit(i & (1 << 0) != 0),
        ])
    }

    fn make_bit(b: bool) -> Bit {
        if b {
            Bit::Positive
        } else {
            Bit::Negative
        }
    }
}

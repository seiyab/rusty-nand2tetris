use crate::general::Zero;

use super::sequential_circuit::SequentialCircuit;

pub struct ArraySC3<A: SequentialCircuit>([A; 3]);

impl<A: SequentialCircuit + Zero> Zero for ArraySC3<A> {
    fn new() -> Self {
        Self([A::new(), A::new(), A::new()])
    }
}

impl<A: SequentialCircuit> SequentialCircuit for ArraySC3<A> {
    type Input = [A::Input; 3];
    type Output = [A::Output; 3];

    fn tick(&self, input: &Self::Input) -> (Self::Output, Self) {
        let Self(a) = self;
        let (o0, s0) = a[0].tick(&input[0]);
        let (o1, s1) = a[1].tick(&input[1]);
        let (o2, s2) = a[2].tick(&input[2]);
        ([o0, o1, o2], Self([s0, s1, s2]))
    }
}

pub struct ArraySC8<A: SequentialCircuit>([A; 8]);

impl<A: SequentialCircuit + Zero> Zero for ArraySC8<A> {
    fn new() -> Self {
        Self([
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
        ])
    }
}

impl<A: SequentialCircuit> SequentialCircuit for ArraySC8<A> {
    type Input = [A::Input; 8];
    type Output = [A::Output; 8];

    fn tick(&self, input: &Self::Input) -> (Self::Output, Self) {
        let Self(a) = self;
        let (o0, s0) = a[0].tick(&input[0]);
        let (o1, s1) = a[1].tick(&input[1]);
        let (o2, s2) = a[2].tick(&input[2]);
        let (o3, s3) = a[3].tick(&input[3]);
        let (o4, s4) = a[4].tick(&input[4]);
        let (o5, s5) = a[5].tick(&input[5]);
        let (o6, s6) = a[6].tick(&input[6]);
        let (o7, s7) = a[7].tick(&input[7]);
        (
            [o0, o1, o2, o3, o4, o5, o6, o7],
            Self([s0, s1, s2, s3, s4, s5, s6, s7]),
        )
    }
}

pub struct ArraySC16<A: SequentialCircuit>([A; 16]);

impl<A: SequentialCircuit + Zero> Zero for ArraySC16<A> {
    fn new() -> Self {
        Self([
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
            A::new(),
        ])
    }
}

impl<A: SequentialCircuit> SequentialCircuit for ArraySC16<A> {
    type Input = [A::Input; 16];
    type Output = [A::Output; 16];

    fn tick(&self, input: &Self::Input) -> (Self::Output, Self) {
        let Self(a) = self;
        let (o0, s0) = a[0x0].tick(&input[0x0]);
        let (o1, s1) = a[0x1].tick(&input[0x1]);
        let (o2, s2) = a[0x2].tick(&input[0x2]);
        let (o3, s3) = a[0x3].tick(&input[0x3]);
        let (o4, s4) = a[0x4].tick(&input[0x4]);
        let (o5, s5) = a[0x5].tick(&input[0x5]);
        let (o6, s6) = a[0x6].tick(&input[0x6]);
        let (o7, s7) = a[0x7].tick(&input[0x7]);
        let (o8, s8) = a[0x8].tick(&input[0x8]);
        let (o9, s9) = a[0x9].tick(&input[0x9]);
        let (oa, sa) = a[0xa].tick(&input[0xa]);
        let (ob, sb) = a[0xb].tick(&input[0xb]);
        let (oc, sc) = a[0xc].tick(&input[0xc]);
        let (od, sd) = a[0xd].tick(&input[0xd]);
        let (oe, se) = a[0xe].tick(&input[0xe]);
        let (of, sf) = a[0xf].tick(&input[0xf]);
        (
            [
                o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, oa, ob, oc, od, oe, of,
            ],
            Self([
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, sa, sb, sc, sd, se, sf,
            ]),
        )
    }
}

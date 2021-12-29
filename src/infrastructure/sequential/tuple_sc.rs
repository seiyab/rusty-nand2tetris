use super::sequential_circuit::SequentialCircuit;

pub struct TupleSC2<A: SequentialCircuit, B: SequentialCircuit>(A, B);

impl<A: SequentialCircuit, B: SequentialCircuit> SequentialCircuit for TupleSC2<A, B> {
    type Input = (A::Input, B::Input);
    type Output = (A::Output, B::Output);

    fn tick(&self, input: Self::Input) -> (Self::Output, Self) {
        let TupleSC2(a, b) = self;
        let (ia, ib) = input;
        let (oa, new_a) = a.tick(ia);
        let (ob, new_b) = b.tick(ib);
        ((oa, ob), TupleSC2(new_a, new_b))
    }
}

pub struct TupleSC3<A: SequentialCircuit, B: SequentialCircuit, C: SequentialCircuit>(A, B, C);

impl<A: SequentialCircuit, B: SequentialCircuit, C: SequentialCircuit> SequentialCircuit
    for TupleSC3<A, B, C>
{
    type Input = (A::Input, B::Input, C::Input);
    type Output = (A::Output, B::Output, C::Output);

    fn tick(&self, input: Self::Input) -> (Self::Output, Self) {
        let TupleSC3(a, b, c) = self;
        let (ia, ib, ic) = input;
        let (oa, new_sa) = a.tick(ia);
        let (ob, new_sb) = b.tick(ib);
        let (oc, new_sc) = c.tick(ic);
        ((oa, ob, oc), TupleSC3(new_sa, new_sb, new_sc))
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn tuple_sc2_works() {
        let a = FuncSC::of(0, &|&s: &i64, _: ()| (s, s + 1));
        let b = FuncSC::of(1, &|&s: &i64, _: ()| (s, s * 2));
        let i = ((), ());
        let t = TupleSC2(a, b);
        let ((oa, ob), t) = t.tick(i);
        assert_eq!((oa, ob), (0, 1));
        let ((oa, ob), t) = t.tick(i);
        assert_eq!((oa, ob), (1, 2));
        let ((oa, ob), t) = t.tick(i);
        assert_eq!((oa, ob), (2, 4));
        let ((oa, ob), _) = t.tick(i);
        assert_eq!((oa, ob), (3, 8));
    }

    #[test]
    fn tuple_sc3_works() {
        let a = FuncSC::of(0, &|s: &i64, i: i64| (s + i, s + i));
        let b = FuncSC::of(1, &|s: &i64, i: i64| (s * i, s * i));
        let c = FuncSC::of(1, &|s: &i64, i: i64| (s - i, s - i));
        let t = TupleSC3(a, b, c);
        let ((oa, ob, oc), t) = t.tick((0, 1, 0));
        assert_eq!((oa, ob, oc), (0, 1, 1));
        let ((oa, ob, oc), t) = t.tick((1, 2, 1));
        assert_eq!((oa, ob, oc), (1, 2, 0));
        let ((oa, ob, oc), _) = t.tick((3, 2, 1));
        assert_eq!((oa, ob, oc), (4, 4, -1));
    }
}

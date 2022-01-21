use core::marker::PhantomData;

pub trait Parser<Atom, Output> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<(Output, usize)>;
}

pub struct AtomParser<A: Eq + Clone>(A);

impl<A: Eq + Clone> Parser<A, A> for AtomParser<A> {
    fn parse(&self, tape: &Vec<A>, position: usize) -> Option<(A, usize)> {
        let next = tape.get(position)?;
        if next == &self.0 {
            Some((next.clone(), position + 1))
        } else {
            None
        }
    }
}

impl<A: Eq + Clone> AtomParser<A> {
    pub fn new(a: A) -> Self {
        Self(a)
    }
}

// Enum / Either 2
#[derive(Debug, Eq, PartialEq)]
pub enum E2<A, B> {
    A(A),
    B(B),
}

pub struct E2Parser<'t, Atom, A, B> {
    a_parser: Box<dyn 't + Parser<Atom, A>>,
    b_parser: Box<dyn 't + Parser<Atom, B>>,
}

impl<'t, Atom, A, B> Parser<Atom, E2<A, B>> for E2Parser<'t, Atom, A, B> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<(E2<A, B>, usize)> {
        if let Some((a, p)) = self.a_parser.parse(&tape, position) {
            Some((E2::A(a), p))
        } else if let Some((b, p)) = self.b_parser.parse(&tape, position) {
            Some((E2::B(b), p))
        } else {
            None
        }
    }
}

impl<'t, Atom, A, B> E2Parser<'t, Atom, A, B> {
    pub fn new<X, Y>(a_parser: X, b_parser: Y) -> Self
    where
        X: 't + Parser<Atom, A>,
        Y: 't + Parser<Atom, B>,
    {
        Self {
            a_parser: Box::new(a_parser),
            b_parser: Box::new(b_parser),
        }
    }
}

pub struct T2Parser<'t, Atom, A, B> {
    a_parser: Box<dyn 't + Parser<Atom, A>>,
    b_parser: Box<dyn 't + Parser<Atom, B>>,
}

impl<'t, Atom, A, B> Parser<Atom, (A, B)> for T2Parser<'t, Atom, A, B> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<((A, B), usize)> {
        let (a, p) = self.a_parser.parse(&tape, position)?;
        let (b, p) = self.b_parser.parse(&tape, p)?;
        Some(((a, b), p))
    }
}

impl<'t, Atom, A, B> T2Parser<'t, Atom, A, B> {
    pub fn new<X, Y>(a_parser: X, b_parser: Y) -> Self
    where
        X: 't + Parser<Atom, A>,
        Y: 't + Parser<Atom, B>,
    {
        Self {
            a_parser: Box::new(a_parser),
            b_parser: Box::new(b_parser),
        }
    }
}

pub struct T3Parser<'t, Atom, A, B, C> {
    a_parser: Box<dyn 't + Parser<Atom, A>>,
    b_parser: Box<dyn 't + Parser<Atom, B>>,
    c_parser: Box<dyn 't + Parser<Atom, C>>,
}

impl<'t, Atom, A, B, C> Parser<Atom, (A, B, C)> for T3Parser<'t, Atom, A, B, C> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<((A, B, C), usize)> {
        let (a, p) = self.a_parser.parse(&tape, position)?;
        let (b, p) = self.b_parser.parse(&tape, p)?;
        let (c, p) = self.c_parser.parse(&tape, p)?;
        Some(((a, b, c), p))
    }
}

impl<'t, Atom, A, B, C> T3Parser<'t, Atom, A, B, C> {
    pub fn new<X, Y, Z>(a_parser: X, b_parser: Y, c_parser: Z) -> Self
    where
        X: 't + Parser<Atom, A>,
        Y: 't + Parser<Atom, B>,
        Z: 't + Parser<Atom, C>,
    {
        Self {
            a_parser: Box::new(a_parser),
            b_parser: Box::new(b_parser),
            c_parser: Box::new(c_parser),
        }
    }
}

pub struct VecParser<Atom, T> {
    parsers: Vec<Box<dyn Parser<Atom, T>>>,
}

impl<Atom, T> Parser<Atom, Vec<T>> for VecParser<Atom, T> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<(Vec<T>, usize)> {
        let mut result = (Vec::new(), position);
        for p in self.parsers.iter() {
            let z = p.parse(&tape, result.1)?;
            result.1 = z.1;
            result.0.push(z.0);
        }
        Some(result)
    }
}

impl<Atom, T> VecParser<Atom, T> {
    pub fn new(parsers: Vec<Box<dyn Parser<Atom, T>>>) -> Self {
        Self { parsers }
    }
}

pub struct OneOfParser<Atom, T> {
    parsers: Vec<Box<dyn Parser<Atom, T>>>,
}

impl<Atom, T> Parser<Atom, T> for OneOfParser<Atom, T> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<(T, usize)> {
        for p in self.parsers.iter() {
            let z = p.parse(&tape, position);
            if z.is_some() {
                return z;
            }
        }
        None
    }
}

impl<Atom, T> OneOfParser<Atom, T> {
    pub fn new(parsers: Vec<Box<dyn Parser<Atom, T>>>) -> Self {
        Self { parsers }
    }
}

pub struct RefinedParser<'a, Atom, P, R> {
    original: Box<dyn 'a + Parser<Atom, P>>,
    refine: Box<dyn 'a + Fn(P) -> R>,
}

impl<'a, Atom, P, R> Parser<Atom, R> for RefinedParser<'a, Atom, P, R> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<(R, usize)> {
        let (o, p) = self.original.parse(&tape, position)?;
        Some(((self.refine)(o), p))
    }
}

impl<'a, Atom, P, R> RefinedParser<'a, Atom, P, R> {
    pub fn new<X, F>(original: X, refine: F) -> Self
    where
        X: 'a + Parser<Atom, P>,
        F: 'a + Fn(P) -> R,
    {
        Self {
            original: Box::new(original),
            refine: Box::new(refine),
        }
    }
}

pub struct MustParser<'a, Atom, T> {
    original: Box<dyn 'a + Parser<Atom, Option<T>>>,
}

impl<'a, Atom, T> Parser<Atom, T> for MustParser<'a, Atom, T> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<(T, usize)> {
        let (o, p) = self.original.parse(&tape, position)?;
        let c = o?;
        Some((c, p))
    }
}

impl<'a, Atom, T> MustParser<'a, Atom, T> {
    pub fn new<X>(original: X) -> Self
    where
        X: 'a + Parser<Atom, Option<T>>,
    {
        Self {
            original: Box::new(original),
        }
    }
}

pub struct FuncParser<'t, Atom, R> {
    func: Box<dyn 't + Fn(&Atom) -> Option<R>>,
}

impl<'t, Atom, R> Parser<Atom, R> for FuncParser<'t, Atom, R> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<(R, usize)> {
        let a = tape.get(position)?;
        let r = (self.func)(a)?;
        Some((r, position + 1))
    }
}

impl<'t, Atom, R> FuncParser<'t, Atom, R> {
    pub fn new<F>(func: F) -> Self
    where
        F: 't + Fn(&Atom) -> Option<R>,
    {
        Self {
            func: Box::new(func),
        }
    }
}

pub struct OptionParser<'t, Atom, T> {
    parser: Box<dyn 't + Parser<Atom, T>>,
}

impl<'t, Atom, T> Parser<Atom, Option<T>> for OptionParser<'t, Atom, T> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<(Option<T>, usize)> {
        if let Some((o, pos)) = self.parser.parse(tape, position) {
            Some((Some(o), pos))
        } else {
            Some((None, position))
        }
    }
}

impl<'t, Atom, T> OptionParser<'t, Atom, T> {
    pub fn new<P>(parser: P) -> Self
    where
        P: 't + Parser<Atom, T>,
    {
        Self {
            parser: Box::new(parser),
        }
    }
}

pub struct RepeatParser<'t, Atom, T> {
    parser: Box<dyn 't + Parser<Atom, T>>,
}

impl<'t, Atom, T> Parser<Atom, Vec<T>> for RepeatParser<'t, Atom, T> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<(Vec<T>, usize)> {
        let mut result = Vec::new();
        let mut p = position;
        loop {
            if let Some((o, pos)) = self.parser.parse(tape, p) {
                result.push(o);
                p = pos;
            } else {
                break;
            }
        }
        Some((result, p))
    }
}

impl<'t, Atom, T> RepeatParser<'t, Atom, T> {
    pub fn new<P>(parser: P) -> Self
    where
        P: 't + Parser<Atom, T>,
    {
        Self {
            parser: Box::new(parser),
        }
    }
}

pub struct DiscardParser<'t, Atom, T> {
    parser: Box<dyn 't + Parser<Atom, T>>,
}

impl<'t, Atom, T> Parser<Atom, ()> for DiscardParser<'t, Atom, T> {
    fn parse(&self, tape: &Vec<Atom>, position: usize) -> Option<((), usize)> {
        let (_, pos) = self.parser.parse(tape, position)?;
        Some(((), pos))
    }
}

impl<'t, Atom, T> DiscardParser<'t, Atom, T> {
    pub fn new<P>(parser: P) -> Self
    where
        P: 't + Parser<Atom, T>,
    {
        Self {
            parser: Box::new(parser),
        }
    }
}

pub struct NoneParser<Atom>(PhantomData<Atom>);

impl<Atom> Parser<Atom, ()> for NoneParser<Atom> {
    fn parse(&self, _: &Vec<Atom>, position: usize) -> Option<((), usize)> {
        Some(((), position))
    }
}

impl<Atom> NoneParser<Atom> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_parser_works() {
        let chars = "abcdef".chars().collect::<Vec<_>>();

        let abc_parser = VecParser::new(vec![
            Box::new(AtomParser('a')),
            Box::new(AtomParser('b')),
            Box::new(AtomParser('c')),
        ]);

        assert_matches!(abc_parser.parse(&chars, 0), Some((_, 3)));

        let def_parser = VecParser::new(vec![
            Box::new(AtomParser('d')),
            Box::new(AtomParser('e')),
            Box::new(AtomParser('f')),
        ]);
        assert_matches!(def_parser.parse(&chars, 0), None);
    }

    #[test]
    fn refined_parser_works() {
        let chars = "abcdef".chars().collect::<Vec<_>>();

        let abc_parser = RefinedParser::new(
            VecParser::new(vec![
                Box::new(AtomParser('a')),
                Box::new(AtomParser('b')),
                Box::new(AtomParser('c')),
            ]),
            Box::new(|_| "abc".to_string()),
        );

        assert_matches!(abc_parser.parse(&chars, 0), Some((_, 3)));
        assert_eq!(abc_parser.parse(&chars, 0).unwrap().0, "abc".to_string());
    }

    #[test]
    fn repeat_parser_works() {
        let chars = "aaaabbc".chars().collect::<Vec<_>>();
        let as_parser = RepeatParser::new(AtomParser::new('a'));
        let bs_parser = RepeatParser::new(AtomParser::new('b'));

        assert_matches!(as_parser.parse(&chars, 0), Some((_, 4)));
        assert_matches!(bs_parser.parse(&chars, 0), Some((_, 0)));
        assert_matches!(bs_parser.parse(&chars, 4), Some((_, 6)));
    }
}

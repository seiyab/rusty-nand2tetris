use crate::parser::{
    AtomParser, FuncParser, Parser, RefinedParser, RepeatParser, T2Parser, VecParser,
};

pub fn word_parser(word: &str) -> RefinedParser<'static, char, Vec<char>, String> {
    let vps: Vec<Box<dyn Parser<char, char>>> = word
        .chars()
        .map(|c: char| Box::new(AtomParser::new(c)) as _)
        .collect();
    let word_string = word.to_string();
    RefinedParser::new(
        VecParser::new(vps),
        Box::new(move |_: _| word_string.clone()),
    )
}

pub fn numeric_parser() -> RefinedParser<'static, char, (u32, Vec<u32>), i32> {
    RefinedParser::new(
        T2Parser::new(
            FuncParser::new(|c: &char| c.to_digit(10)),
            RepeatParser::new(FuncParser::new(|c: &char| c.to_digit(10))),
        ),
        |(x, ys)| {
            let mut result = x;
            for y in ys.iter() {
                result *= 10;
                result += y;
            }
            result as i32
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn word_parser_works() {
        let text = "abcdef".chars().collect::<Vec<_>>();
        let abc_parser = word_parser(&"abc");
        let def_parser = word_parser(&"def");

        assert_eq!(abc_parser.parse(&text, 0), Some(("abc".to_string(), 3)));
        assert_eq!(def_parser.parse(&text, 0), None);
        assert_eq!(def_parser.parse(&text, 3), Some(("def".to_string(), 6)));
    }

    #[test]
    fn numeric_parser_works() {
        let text = "12345".chars().collect::<Vec<_>>();
        assert_eq!(numeric_parser().parse(&text, 0), Some((12345, 5)));
        assert_eq!(numeric_parser().parse(&text, 1), Some((2345, 5)));
    }
}

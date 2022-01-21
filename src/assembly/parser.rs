use crate::instruction;
use crate::parser::text_parser::{numeric_parser, word_parser};
use crate::parser::{
    AtomParser, DiscardParser, E2Parser, FuncParser, MustParser, OneOfParser, OptionParser, Parser,
    RefinedParser, RepeatParser, T2Parser, T3Parser, E2,
};

use super::asm::*;

pub fn parse(code: &str) -> Result<Vec<E2<AsmInst, Label>>, String> {
    let (x, p) = asm_parser()
        .parse(&code.chars().collect(), 0)
        .ok_or(String::from("failed to parse"))?;
    if p == code.len() {
        return Ok(x);
    }
    Err(String::from("character remains"))
}

fn asm_parser(
) -> RefinedParser<'static, char, Vec<Option<E2<AsmInst, Label>>>, Vec<E2<AsmInst, Label>>> {
    RefinedParser::new(
        RepeatParser::new(RefinedParser::new(
            T2Parser::new(line_parser(), AtomParser::new('\n')),
            |(l, _)| l,
        )),
        |mut v| v.drain(..).filter_map(|x| x).collect(),
    )
}

fn line_parser(
) -> RefinedParser<'static, char, ((), Option<E2<AsmInst, Label>>, ()), Option<E2<AsmInst, Label>>>
{
    RefinedParser::new(
        T3Parser::new(
            DiscardParser::new(RepeatParser::new(AtomParser::new(' '))),
            OptionParser::new(E2Parser::new(instruction_parser(), label_parser())),
            DiscardParser::new(T2Parser::new(
                RepeatParser::new(AtomParser::new(' ')),
                OptionParser::new(comment_parser()),
            )),
        ),
        |(_, e, _)| e,
    )
}

fn instruction_parser() -> RefinedParser<'static, char, E2<A, instruction::Computation>, AsmInst> {
    RefinedParser::new(
        E2Parser::new(a_instruction_parser(), c_instruction_parser()),
        |e2| match e2 {
            E2::A(a) => AsmInst::A(a),
            E2::B(c) => AsmInst::C(c),
        },
    )
}

fn a_instruction_parser() -> RefinedParser<'static, char, (char, E2<i32, String>), A> {
    RefinedParser::new(
        T2Parser::new(
            AtomParser::new('@'),
            E2Parser::new(numeric_parser(), symbol_parser()),
        ),
        Box::new(|(_, x)| match x {
            E2::A(n) => A::Const(n),
            E2::B(s) => A::Var(s),
        }),
    )
}

fn c_instruction_parser() -> RefinedParser<
    'static,
    char,
    (
        Option<instruction::Dest>,
        (instruction::CompReg, instruction::Comp),
        Option<instruction::Jump>,
    ),
    instruction::Computation,
> {
    RefinedParser::new(
        T3Parser::new(
            OptionParser::new(dest_parser()),
            comp_parser(),
            OptionParser::new(jump_parser()),
        ),
        |(d, comp, j)| {
            let dest = d.unwrap_or(instruction::Dest::None);
            let jump = j.unwrap_or(instruction::Jump::None);
            instruction::Computation { dest, comp, jump }
        },
    )
}

fn label_parser() -> RefinedParser<'static, char, (char, String, char), Label> {
    RefinedParser::new(
        T3Parser::new(AtomParser::new('('), symbol_parser(), AtomParser::new(')')),
        |(_, s, _)| Label(s),
    )
}

fn symbol_parser() -> RefinedParser<'static, char, (char, Vec<char>), String> {
    RefinedParser::new(
        T2Parser::new(
            FuncParser::new(|c: &char| {
                if c.is_ascii_alphabetic() {
                    Some(*c)
                } else {
                    match c {
                        '_' | '.' | '$' | ':' => Some(*c),
                        _ => None,
                    }
                }
            }),
            RepeatParser::new(FuncParser::new(|c: &char| {
                if c.is_ascii_alphabetic() || c.is_numeric() {
                    Some(*c)
                } else {
                    match c {
                        '_' | '.' | '$' | ':' => Some(*c),
                        _ => None,
                    }
                }
            })),
        ),
        |(c, cs)| String::from(c) + cs.iter().collect::<String>().as_str(),
    )
}

fn dest_parser() -> MustParser<'static, char, instruction::Dest> {
    MustParser::new(RefinedParser::new(
        T2Parser::new(
            RepeatParser::new(OneOfParser::new(
                vec!['A', 'M', 'D']
                    .iter()
                    .map(|&c| Box::new(AtomParser::new(c)) as _)
                    .collect(),
            )),
            AtomParser::new('='),
        ),
        |(v, _)| match v.iter().collect::<String>().as_str() {
            "A" => Some(instruction::Dest::A),
            "AD" => Some(instruction::Dest::AD),
            "AM" => Some(instruction::Dest::AM),
            "ADM" => Some(instruction::Dest::ADM),
            "D" => Some(instruction::Dest::D),
            "DM" => Some(instruction::Dest::DM),
            "M" => Some(instruction::Dest::M),
            _ => None,
        },
    ))
}

fn comp_parser() -> MustParser<'static, char, (instruction::CompReg, instruction::Comp)> {
    use instruction::Comp as C;
    use instruction::CompReg as CR;
    MustParser::new(RefinedParser::new(
        RepeatParser::new(OneOfParser::new(
            vec!['0', '1', '+', '-', '!', '&', '|', 'A', 'M', 'D']
                .iter()
                .map(|&c| Box::new(AtomParser::new(c)) as _)
                .collect(),
        )),
        |v| match v.iter().collect::<String>().as_str() {
            "0" => Some((CR::A, C::Zero)),
            "1" => Some((CR::A, C::One)),
            "-1" => Some((CR::A, C::MinusOne)),
            "D" => Some((CR::A, C::D)),
            "A" => Some((CR::A, C::A)),
            "!D" => Some((CR::A, C::NotD)),
            "!A" => Some((CR::A, C::NotA)),
            "D+1" => Some((CR::A, C::DPlus1)),
            "A+1" => Some((CR::A, C::APlus1)),
            "D-1" => Some((CR::A, C::DMinus1)),
            "A-1" => Some((CR::A, C::AMinus1)),
            "D+A" => Some((CR::A, C::DPlusA)),
            "D-A" => Some((CR::A, C::DMinusA)),
            "A-D" => Some((CR::A, C::AMinusD)),
            "D&A" => Some((CR::A, C::DAndA)),
            "D|A" => Some((CR::A, C::DOrA)),
            "M" => Some((CR::M, C::A)),
            "!M" => Some((CR::M, C::NotA)),
            "-M" => Some((CR::M, C::MinusA)),
            "D+M" => Some((CR::M, C::DPlusA)),
            "D-M" => Some((CR::M, C::DMinusA)),
            "M-D" => Some((CR::M, C::AMinusD)),
            "D&M" => Some((CR::M, C::DAndA)),
            "D|M" => Some((CR::M, C::DOrA)),
            _ => None,
        },
    ))
}

fn jump_parser() -> MustParser<'static, char, instruction::Jump> {
    MustParser::new(RefinedParser::new(
        T2Parser::new(
            AtomParser::new(';'),
            RepeatParser::new(FuncParser::new(|c: &char| {
                if c.is_ascii_alphabetic() {
                    Some(*c)
                } else {
                    None
                }
            })),
        ),
        |(_, v)| match v.iter().collect::<String>().as_str() {
            "JGT" => Some(instruction::Jump::Gt),
            "JEQ" => Some(instruction::Jump::Eq),
            "JGE" => Some(instruction::Jump::Ge),
            "JLT" => Some(instruction::Jump::Lt),
            "JNE" => Some(instruction::Jump::Ne),
            "JLE" => Some(instruction::Jump::Le),
            "JMP" => Some(instruction::Jump::Always),
            _ => None,
        },
    ))
}

fn comment_parser() -> DiscardParser<'static, char, (String, Vec<()>)> {
    DiscardParser::new(T2Parser::new(
        word_parser("//"),
        RepeatParser::new(FuncParser::new(
            |&c| if c == '\n' { None } else { Some(()) },
        )),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::{Dest, Jump};
    use crate::parser::Parser;

    #[test]
    fn a_instruction_parser_works() {
        let text = "@123\n\
        @45\n\
        A=D\n"
            .chars()
            .collect::<Vec<_>>();
        let p = a_instruction_parser();

        assert_matches!(p.parse(&text, 0), Some((A::Const(123), 4)));
        assert_matches!(p.parse(&text, 5), Some((A::Const(45), 8)));
        assert_matches!(p.parse(&text, 9), None);

        let text = "@VAR".chars().collect::<Vec<_>>();
        assert_eq!(p.parse(&text, 0), Some((A::Var(String::from("VAR")), 4)))
    }

    #[test]
    fn c_instruction_parser_works() {
        use instruction::*;

        let text = "A=D\n\
        M=A+1\n\
        !D;JGT\n\
        AD=D-M"
            .chars()
            .collect::<Vec<_>>();
        let p = c_instruction_parser();

        assert_eq!(
            p.parse(&text, 0),
            Some((
                Computation {
                    dest: Dest::A,
                    comp: (CompReg::A, Comp::D),
                    jump: Jump::None
                },
                3
            ))
        );
        assert_eq!(
            p.parse(&text, 4),
            Some((
                Computation {
                    dest: Dest::M,
                    comp: (CompReg::A, Comp::APlus1),
                    jump: Jump::None
                },
                9
            ))
        );
        assert_eq!(
            p.parse(&text, 10),
            Some((
                Computation {
                    dest: Dest::None,
                    comp: (CompReg::A, Comp::NotD),
                    jump: Jump::Gt
                },
                16
            ))
        );
        assert_eq!(
            p.parse(&text, 17),
            Some((
                Computation {
                    dest: Dest::AD,
                    comp: (CompReg::M, Comp::DMinusA),
                    jump: Jump::None
                },
                23
            ))
        );
    }

    #[test]
    fn line_parser_works() {
        use instruction::*;

        let text = "A=D".chars().collect::<Vec<_>>();
        let p = line_parser();

        assert_matches!(
            p.parse(&text, 0),
            Some((
                Some(E2::A(AsmInst::C(Computation {
                    dest: Dest::A,
                    comp: (CompReg::A, Comp::D),
                    jump: Jump::None
                }))),
                3
            ))
        );

        let text = "@100".chars().collect::<Vec<_>>();
        let p = line_parser();
        assert_matches!(
            p.parse(&text, 0),
            Some((Some(E2::A(AsmInst::A(A::Const(100)))), 4))
        );

        let text = "// comment".chars().collect::<Vec<_>>();
        let p = line_parser();
        assert_matches!(p.parse(&text, 0), Some((None, 10)));

        let text = "@57 // comment".chars().collect::<Vec<_>>();
        let p = line_parser();
        assert_matches!(
            p.parse(&text, 0),
            Some((Some(E2::A(AsmInst::A(A::Const(57)))), 14))
        );

        let text = "@SYMBOL".chars().collect::<Vec<_>>();
        let p = line_parser();
        assert_eq!(
            p.parse(&text, 0),
            Some((Some(E2::A(AsmInst::A(A::Var(String::from("@SYMBOL"))))), 7))
        );
    }
}

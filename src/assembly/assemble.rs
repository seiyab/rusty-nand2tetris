use std::collections::HashMap;

use crate::assembly::parser::parse;
use crate::gates::bus16::Bus16;
use crate::instruction::Instruction;
use crate::parser::E2;

use super::asm::*;

pub fn assemble(code: &str) -> Result<Vec<Bus16>, String> {
    let lines = parse(code)?;
    let mut t = label_table(&lines)?;
    t.extend(var_table(&lines, &t));
    let main = resolve(lines, &t).iter().map(|inst| inst.bus16()).collect();
    Ok(main)
}

fn label_table(lines: &Vec<E2<AsmInst, Label>>) -> Result<HashMap<String, i32>, String> {
    let mut table = HashMap::new();
    let mut pc = 0;
    for line in lines {
        match line {
            E2::A(_) => pc += 1,
            E2::B(Label(label)) => {
                table
                    .try_insert(label.clone(), pc)
                    .map_err(|e| e.entry.key().clone())?;
            }
        }
    }
    Ok(table)
}

fn var_table(
    lines: &Vec<E2<AsmInst, Label>>,
    labels: &HashMap<String, i32>,
) -> HashMap<String, i32> {
    let mut table = HashMap::new();
    let mut addr = 0x10;
    for line in lines {
        if let E2::A(AsmInst::A(A::Var(a))) = line {
            if labels.contains_key(a) {
                continue;
            }
            let r = table.try_insert(a.clone(), addr);
            if let Ok(_) = r {
                addr += 1;
            }
        }
    }
    table
}

fn resolve(mut lines: Vec<E2<AsmInst, Label>>, table: &HashMap<String, i32>) -> Vec<Instruction> {
    lines
        .drain(..)
        .filter_map(|e| match e {
            E2::A(a) => match a {
                AsmInst::A(A::Const(c)) => Some(Instruction::A(c)),
                AsmInst::A(A::Var(v)) => table.get(&v).map(|&x| Instruction::A(x)),
                AsmInst::C(c) => Some(Instruction::C(c)),
            },
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::computer::testing::ComputerDebug;
    use crate::computer::MutComputer;
    use crate::infrastructure::sequential::MutSC;

    use super::*;

    #[test]
    fn add_works() {
        let code = "
        @2 \n\
        D=A \n\
        @3 \n\
        D=D+A \n\
        @0 \n\
        M=D \n\
        (END) \n\
        @END \n\
        0;JMP \n";

        let bin = assemble(&code).unwrap();

        let mut c = MutComputer::of(&bin);
        for _ in 0..10 {
            c.tick(&())
        }

        assert_eq!(c.peek_ram(0), 5)
    }

    #[test]
    fn max_works() {
        let code = "
        @0
        D=M
        @1
        D=D-M
        @LESS
        D;JLT
        @0
        D=M
        @2
        M=D
        @END
        0;JMP
        (LESS)
        @1
        D=M
        @2
        M=D
        (END)
        @END
        0;JMP
        \
        \n";

        let prepare = |x: i32, y: i32| {
            format!(
                "
        @{}
        D=A
        @0
        M=D
        @{}
        D=A
        @1
        M=D
        ",
                x, y
            )
        };

        let test_cases = [(1, 0, 1), (10, 15, 15), (6, 8, 8), (100, 0, 100)];

        for &(x, y, ans) in test_cases.iter() {
            let bin = assemble(&(prepare(x, y) + code)).unwrap();

            let mut c = MutComputer::of(&bin);
            for _ in 0..bin.len() {
                c.tick(&())
            }

            assert_eq!(c.peek_ram(2), ans)
        }
    }
}

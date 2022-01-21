use crate::gates::bus16;
use crate::gates::bus16::Bus16;
use crate::general::Zero;
use crate::infrastructure::sequential::*;
use crate::primitive::Bit;

use super::{Cpu, CpuInput, CpuOutput, DataMemoryInput, MutDataMemory, MutRom};

// future work: should be constructed of gates
pub struct MutComputer {
    ram: MutDataMemory,
    rom: MutRom,
    cpu: Cpu,
}

impl MutSC for MutComputer {
    type Input = ();
    type Output = ();

    fn tick(&mut self, _: &Self::Input) -> Self::Output {
        let (
            CpuOutput {
                pc, address_m: a, ..
            },
            _,
        ) = self.cpu.tick(&CpuInput {
            in_m: Bus16::new(),
            instruction: Bus16::new(),
            reset: Bit::new(),
        });
        let instruction = self.rom.tick(&pc);
        let address_m = bus16::into_bus13(&a);
        let in_m = self.ram.tick(&DataMemoryInput {
            input: Bus16::new(),
            load: Bit::Negative,
            address: address_m.clone(),
        });
        let (o, new_cpu) = self.cpu.tick(&CpuInput {
            in_m,
            instruction,
            reset: Bit::Negative,
        });
        self.cpu = new_cpu;
        self.ram.tick(&DataMemoryInput {
            input: o.out_m,
            load: o.write_m,
            address: address_m,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::gates::bus16::testing::into_i32;
    use crate::instruction::*;

    use super::super::cpu::testing::CpuDebug;
    use super::testing::ComputerDebug;
    use super::*;

    #[test]
    fn load_a() {
        let is = vec![Instruction::A(1000).bus16()];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }
        let a = c.cpu.peek_address();
        assert_eq!(into_i32(&a), 1000);
    }

    #[test]
    fn load_d() {
        let is = vec![
            Instruction::A(999).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }
        let d = c.cpu.peek_data();
        assert_eq!(into_i32(&d), 999);
    }

    #[test]
    fn simple_store() {
        let is = vec![
            Instruction::A(100).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(0).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }
        let ans = c.ram.peek(0);
        assert_eq!(into_i32(&ans), 100);
    }

    #[test]
    fn a_to_memory_to_d() {
        let is = vec![
            Instruction::A(57).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(11).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(0).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(11).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }
        assert_eq!(into_i32(&c.cpu.peek_data()), 57);
    }

    #[test]
    fn simple_add() {
        let is = vec![
            Instruction::A(3).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(5).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::DPlusA),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(10).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }

        let ans = c.ram.peek(10);
        assert_eq!(into_i32(&ans), 8);
    }

    #[test]
    fn d_plus_m() {
        let is = vec![
            Instruction::A(5).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(0).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(9).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(0).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::DPlusA),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }

        assert_eq!(into_i32(&c.ram.peek(0)), 14);
    }

    #[test]
    fn d_minus_m() {
        let is = vec![
            Instruction::A(5).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(0).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(9).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(0).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::DMinusA),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }

        let ans = c.ram.peek(0);
        assert_eq!(into_i32(&ans), 4);
    }

    #[test]
    fn increment_address() {
        let is = vec![
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::APlus1),
                dest: Dest::A,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::APlus1),
                dest: Dest::A,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::APlus1),
                dest: Dest::A,
                jump: Jump::None,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }

        assert_eq!(into_i32(&c.cpu.peek_address()), 3);
    }

    #[test]
    fn jump_eq() {
        let is = vec![
            Instruction::A(100).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::Zero),
                dest: Dest::None,
                jump: Jump::Eq,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }

        assert_eq!(into_i32(&c.cpu.peek_pc()), 100);
    }

    #[test]
    fn no_jump_eq() {
        let is = vec![
            Instruction::A(100).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::None,
                jump: Jump::Eq,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }

        assert_eq!(into_i32(&c.cpu.peek_pc()), 2);
    }

    #[test]
    fn no_jump_neq() {
        let is = vec![
            Instruction::A(100).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::Zero),
                dest: Dest::None,
                jump: Jump::NEq,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }

        assert_eq!(into_i32(&c.cpu.peek_pc()), 2);
    }

    #[test]
    fn increment_memory() {
        let is = vec![
            Instruction::A(1).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::APlus1),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::APlus1),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::APlus1),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::APlus1),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..is.len() {
            c.tick(&());
        }

        let ans = c.ram.peek(1);
        assert_eq!(into_i32(&ans), 4);
    }

    #[test]
    fn increment_loop() {
        let end = 100;
        let is = vec![
            Instruction::A(10).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(0).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::APlus1),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::DMinusA),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(),
            Instruction::A(0).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::None,
                jump: Jump::NEq,
            })
            .bus16(),
            Instruction::A(end).bus16(),
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::Zero),
                dest: Dest::None,
                jump: Jump::Always,
            })
            .bus16(),
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..100 {
            c.tick(&());
            let pc = into_i32(&c.cpu.peek_pc());
            if pc >= end {
                break;
            }
        }
        assert_eq!(into_i32(&c.cpu.peek_pc()), end);

        assert_eq!(into_i32(&c.ram.peek(0)), 10);
    }

    #[test]
    fn sum_up() {
        let i = 0;
        let sum = 1;
        let end = 100;
        let is = vec![
            // i=1
            Instruction::A(1).bus16(), // 0
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(), // 1
            Instruction::A(i).bus16(), // 2
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(), // 3
            //
            // sum=0
            Instruction::A(0).bus16(), // 4
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(), // 5
            Instruction::A(sum).bus16(), // 6
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(), // 7
            //
            // D = i
            Instruction::A(i).bus16(), // 8
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(), // 9
            //
            // GOTO end if i > 10
            Instruction::A(10).bus16(), // 10
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::DMinusA),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(), // 11
            Instruction::A(end).bus16(), // 12
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::D),
                dest: Dest::None,
                jump: Jump::Gt,
            })
            .bus16(), // 13
            //
            // D = i
            Instruction::A(i).bus16(), // 14
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::A),
                dest: Dest::D,
                jump: Jump::None,
            })
            .bus16(), // 15
            //
            // sum = sum + i
            Instruction::A(sum).bus16(), // 16
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::DPlusA),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(), // 17
            //
            // i++
            Instruction::A(i).bus16(), // 18
            Instruction::C(Computation {
                comp: (CompReg::M, Comp::APlus1),
                dest: Dest::M,
                jump: Jump::None,
            })
            .bus16(), // 19
            //
            // goto 8
            Instruction::A(8).bus16(), // 20
            Instruction::C(Computation {
                comp: (CompReg::A, Comp::Zero),
                dest: Dest::None,
                jump: Jump::Always,
            })
            .bus16(), //21
        ];
        let mut c = MutComputer::of(&is);
        for _ in 0..300 {
            c.tick(&());
            let pc = into_i32(&c.cpu.peek_pc());
            if pc >= end {
                break;
            }
        }
        assert_eq!(into_i32(&c.cpu.peek_pc()), end);
        assert_eq!(into_i32(&c.ram.peek(sum)), 55);
    }
}

pub mod testing {
    use crate::gates::bus16::testing::into_i32;

    use super::super::memory::testing;
    use super::*;

    pub trait ComputerDebug {
        fn of(instructions: &Vec<Bus16>) -> Self;
        fn peek_ram(&mut self, addr: i32) -> i32;
    }

    impl ComputerDebug for MutComputer {
        fn of(instructions: &Vec<Bus16>) -> Self {
            MutComputer {
                rom: MutRom::of(instructions),
                ram: MutDataMemory::new(),
                cpu: Cpu::new(),
            }
        }

        fn peek_ram(&mut self, addr: i32) -> i32 {
            into_i32(&self.ram.peek(addr))
        }
    }
}

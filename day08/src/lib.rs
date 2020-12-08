use aoc2020::parse;

use bitvec::{bitvec, order::Lsb0, vec::BitVec};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
#[display(style = "snake_case")]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
#[display("{operation} {argument}")]
pub struct Instruction {
    operation: Operation,
    argument: i64,
}

pub struct HandheldGameConsole {
    instructions: Vec<Instruction>,
    instruction_pointer: i64,
    accumulator: i64,
    loop_detect: BitVec<Lsb0, u64>,
}

impl HandheldGameConsole {
    /// Initialize a handheld game console
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            loop_detect: bitvec!(Lsb0, u64; 0; instructions.len()),
            instructions,
            instruction_pointer: 0,
            accumulator: 0,
        }
    }

    /// Execute a single instruction
    ///
    /// If this instruction has previously been seen, return `true` without executing it.
    /// Once this function returns `true`, further calls are idempotent.
    fn step(&mut self) -> Result<bool, Error> {
        if !(0..self.instructions.len() as i64).contains(&self.instruction_pointer) {
            return Err(Error::InstructionPointerOutOfRange(
                self.instruction_pointer,
                self.instructions.len(),
            ));
        }
        let ip = self.instruction_pointer as usize;
        if *self
            .loop_detect
            .get(ip)
            .expect("instructions initialized with appropriate len; qed")
        {
            return Ok(true);
        }
        self.loop_detect.set(ip, true);
        let instruction = self.instructions[ip];
        let delta_ip = match instruction.operation {
            Operation::Acc => {
                self.accumulator += instruction.argument;
                1
            }
            Operation::Jmp => instruction.argument,
            Operation::Nop => 1,
        };
        self.instruction_pointer += delta_ip;

        Ok(false)
    }

    /// Run this computer until a loop is detected.
    ///
    /// Return the current value of the accumulator on loop.
    pub fn run(&mut self) -> Result<i64, Error> {
        while !self.step()? {}
        Ok(self.accumulator)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let instructions: Vec<Instruction> = parse(input)?.collect();
    let mut computer = HandheldGameConsole::new(instructions);
    let acc = computer.run()?;
    println!("accumulator on loop: {}", acc);
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("instruction pointer out of range: must be in 0..{1}; is {0}")]
    InstructionPointerOutOfRange(i64, usize),
}

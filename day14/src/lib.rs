use aoc2020::parse;

use std::{collections::HashMap, fmt, iter::FromIterator, path::Path, str::FromStr};
use thiserror::Error;

const U36_MASK: u64 = 0x0f_ffff_ffff;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Mask {
    zeroes: u64,
    ones: u64,
}

impl Default for Mask {
    fn default() -> Self {
        Mask {
            zeroes: U36_MASK,
            ones: 0,
        }
    }
}

impl fmt::Debug for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mask")
            .field("zeroes", &format!("{:036b}", self.zeroes))
            .field("ones", &format!("{:036b}", self.ones))
            .finish()
    }
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        value & self.zeroes | self.ones
    }
}

impl FromStr for Mask {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            return Err(Error::WrongMaskLen(s.len()));
        }

        let mut zeroes = U36_MASK;
        let mut ones = 0;

        for (idx, ch) in s.chars().rev().enumerate() {
            match ch {
                'X' => {}
                '0' => zeroes &= !(1 << idx),
                '1' => ones |= 1 << idx,
                _ => return Err(Error::UnexpectedMaskChar(ch)),
            }
        }

        Ok(Mask { zeroes, ones })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::FromStr)]
enum Instruction {
    #[display("mask = {0}")]
    Mask(Mask),
    #[display("mem[{idx}] = {value}")]
    Write { idx: usize, value: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct DockingProgram {
    mask: Mask,
    memory: HashMap<usize, u64>,
}

impl FromIterator<Instruction> for DockingProgram {
    fn from_iter<T: IntoIterator<Item = Instruction>>(iter: T) -> Self {
        let mut program = DockingProgram::default();
        for instruction in iter.into_iter() {
            match instruction {
                Instruction::Mask(mask) => program.mask = mask,
                Instruction::Write { idx, value } => {
                    *program.memory.entry(idx).or_default() = program.mask.apply(value);
                }
            }
        }
        program
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let program: DockingProgram = parse(input)?.collect();
    let sum = program.memory.values().sum::<u64>();
    println!("sum of memory values: {}", sum,);
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("wrong mask len; need 36, have {0}")]
    WrongMaskLen(usize),
    #[error("unexpected mask char: {0}")]
    UnexpectedMaskChar(char),
}

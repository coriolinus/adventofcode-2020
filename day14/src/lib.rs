use aoc2020::parse;

use std::{collections::HashMap, fmt, iter::FromIterator, path::Path, str::FromStr};
use thiserror::Error;

const U36_MASK: i64 = 0x0f_ffff_ffff;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Mask {
    zeroes: i64,
    ones: i64,
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
    fn apply(&self, value: i64) -> i64 {
        value & self.zeroes | self.ones
    }

    fn floating_bits(&self) -> i64 {
        !(!self.zeroes | self.ones)
    }

    /// Compute a sequence of floating masks
    fn floating_masks(&self) -> impl Iterator<Item = i64> {
        let floating = self.floating_bits();
        let n_masks = 2_u64.pow(floating.count_ones());
        (0..n_masks).map(move |mut n| {
            // we need to map the rightmost `n_masks` bits of `n` onto `floating`.
            let mut floating_bits_remaining = floating;
            let mut output = 0;

            while n > 0 && floating_bits_remaining > 0 {
                let right_bit = n & 1;
                n >>= 1;

                // now apply `right_bit` to the position of the rightmost 1 in `floating_bits_remaining`
                // bit tricks are fun
                let rightmost_floating_bit = floating_bits_remaining & (-floating_bits_remaining);
                // unset that bit
                floating_bits_remaining &= !rightmost_floating_bit;
                if right_bit > 0 {
                    output |= rightmost_floating_bit;
                }
            }

            output
        })
    }

    fn apply_floating(&self, value: i64) -> impl Iterator<Item = i64> {
        let floating = self.floating_bits();
        let ones = self.ones;
        self.floating_masks()
            .map(move |mask| value & !floating | mask | ones)
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
    Write { idx: usize, value: i64 },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct DockingProgram {
    mask: Mask,
    memory: HashMap<usize, i64>,
}

impl FromIterator<Instruction> for DockingProgram {
    fn from_iter<T: IntoIterator<Item = Instruction>>(iter: T) -> Self {
        let mut program = DockingProgram::default();
        for instruction in iter.into_iter() {
            match instruction {
                Instruction::Mask(mask) => program.mask = mask,
                Instruction::Write { idx, value } => {
                    program.memory.insert(idx, program.mask.apply(value));
                }
            }
        }
        program
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct DockingProgramV2 {
    mask: Mask,
    memory: HashMap<i64, i64>,
}

impl FromIterator<Instruction> for DockingProgramV2 {
    fn from_iter<T: IntoIterator<Item = Instruction>>(iter: T) -> Self {
        let mut program = DockingProgramV2::default();
        for instruction in iter.into_iter() {
            match instruction {
                Instruction::Mask(mask) => program.mask = mask,
                Instruction::Write { idx, value } => {
                    for idx in program.mask.apply_floating(idx as i64) {
                        program.memory.insert(idx, value);
                    }
                }
            }
        }
        program
    }
}

fn print_memory<K, V>(memory: &HashMap<K, V>)
where
    K: Copy + Ord + fmt::Display + fmt::Binary + std::hash::Hash,
    V: Copy + fmt::Display,
{
    let mut keys: Vec<_> = memory.keys().copied().collect();
    keys.sort_unstable();
    for key in keys {
        println!(
            "{key:036b}  (decimal {key}) => {value}",
            key = key,
            value = memory
                .get(&key)
                .copied()
                .expect("this key is known to exist")
        );
    }
}

pub fn part1(input: &Path, show_memory: bool) -> Result<(), Error> {
    let program: DockingProgram = parse(input)?.collect();
    let sum = program.memory.values().sum::<i64>();

    if show_memory {
        print_memory(&program.memory)
    }

    println!("sum of memory values: {}", sum,);
    Ok(())
}

pub fn part2(input: &Path, show_memory: bool) -> Result<(), Error> {
    let program: DockingProgramV2 = parse(input)?.collect();
    let sum = program.memory.values().sum::<i64>();

    if show_memory {
        print_memory(&program.memory)
    }

    println!("sum of memory values: {}", sum);
    Ok(())
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_floating_example_1() {
        let mask: Mask = dbg!("000000000000000000000000000000X1001X".parse().unwrap());
        assert_eq!(mask.floating_bits(), 0b100001);
    }

    #[test]
    fn test_floating_example_2() {
        let mask: Mask = dbg!("00000000000000000000000000000000X0XX".parse().unwrap());
        assert_eq!(mask.floating_bits(), 0b1011);
    }

    #[test]
    fn test_floating_mask_1() {
        let mask: Mask = dbg!("000000000000000000000000000000X1001X".parse().unwrap());
        println!("mask.floating_bits() = {:036b}", mask.floating_bits());
        let expect = [0b_0_0000_0, 0b_0_0000_1, 0b_1_0000_0, 0b_1_0000_1];
        let floating_masks: Vec<_> = mask.floating_masks().collect();
        assert_eq!(&floating_masks, &expect);
    }

    #[test]
    fn test_floating_mask_2() {
        let mask: Mask = dbg!("00000000000000000000000000000000X0XX".parse().unwrap());
        println!("mask.floating_bits() = {:036b}", mask.floating_bits());
        let expect = [
            0b0000, 0b0001, 0b0010, 0b0011, 0b1000, 0b1001, 0b1010, 0b1011,
        ];
        let floating_masks: Vec<_> = mask.floating_masks().collect();
        assert_eq!(&floating_masks, &expect);
    }

    #[test]
    fn test_apply_floating_1() {
        let mask: Mask = dbg!("000000000000000000000000000000X1001X".parse().unwrap());
        let value = 42;
        println!("value =                {:06b}", value);
        let expect = [0b_0_1101_0, 0b_0_1101_1, 0b_1_1101_0, 0b_1_1101_1];
        println!("mask.floating_bits() = {:06b}", mask.floating_bits());
        let floating: Vec<_> = mask.apply_floating(value).collect();
        for (want, have) in expect.iter().zip(&floating) {
            println!("want: {:06b}; have: {:06b}", want, have);
        }
        assert_eq!(&floating, &expect);
    }

    #[test]
    fn test_apply_floating_2() {
        let mask: Mask = dbg!("00000000000000000000000000000000X0XX".parse().unwrap());
        let value = 26;
        let expect = [
            0b_1_0000, 0b_1_0001, 0b_1_0010, 0b_1_0011, 0b_1_1000, 0b_1_1001, 0b_1_1010, 0b_1_1011,
        ];
        let floating: Vec<_> = mask.apply_floating(value).collect();
        assert_eq!(&floating, &expect);
    }
}

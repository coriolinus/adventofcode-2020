use aoc2020::parse;

use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

struct PasswordPolicy {
    min_count: u32,
    max_count: u32,
    char_counted: char,
}

impl FromStr for PasswordPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space_parts: Vec<_> = s.split(' ').collect();
        if space_parts.len() != 2 {
            Err(format!("expected 2 space_parts; got {}", space_parts.len()))?;
        }
        let num_parts: Vec<_> = space_parts[0].split('-').collect();
        if num_parts.len() != 2 {
            Err(format!("expected 2 num_parts; got {}", num_parts.len()))?;
        }

        let min_count = num_parts[0].parse::<u32>().map_err(|err| err.to_string())?;
        let max_count = num_parts[1].parse::<u32>().map_err(|err| err.to_string())?;
        let mut char_policy_chars = space_parts[1].chars();
        let char_counted = char_policy_chars
            .next()
            .ok_or(format!("no char in space_parts[1]"))?;
        if let Some(c) = char_policy_chars.next() {
            Err(format!(
                "expected a single char in char_policy; found extra '{}'",
                c
            ))?;
        }

        Ok(PasswordPolicy {
            min_count,
            max_count,
            char_counted,
        })
    }
}

struct PasswordExample {
    policy: PasswordPolicy,
    example: String,
}

impl FromStr for PasswordExample {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(": ").collect();
        if parts.len() != 2 {
            Err(format!(
                "expected 2 passwordexample parts; got {}",
                parts.len()
            ))?;
        }

        let policy = parts[0].parse()?;
        let example = parts[1].to_string();

        Ok(PasswordExample { policy, example })
    }
}

impl PasswordExample {
    fn is_valid(&self) -> bool {
        let ch_count = self
            .example
            .chars()
            .filter(|ch| *ch == self.policy.char_counted)
            .count();
        ch_count >= self.policy.min_count as usize && ch_count <= self.policy.max_count as usize
    }

    fn is_valid_part2(&self) -> bool {
        let example = self.example.as_bytes();
        (example[self.policy.min_count as usize - 1] == self.policy.char_counted as u8)
            ^ (example[self.policy.max_count as usize - 1] == self.policy.char_counted as u8)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let n_valid = parse::<PasswordExample>(input)?
        .filter(|example| example.is_valid())
        .count();
    println!("{} valid passwords", n_valid);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let n_valid = parse::<PasswordExample>(input)?
        .filter(|example| example.is_valid_part2())
        .count();
    println!("{} valid passwords (part 2)", n_valid);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

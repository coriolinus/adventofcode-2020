use aoc2020::parse;

use std::path::Path;
use thiserror::Error;

#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{min_count}-{max_count} {char_counted}")]
struct PasswordPolicy {
    min_count: u32,
    max_count: u32,
    char_counted: char,
}

#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{policy}: {example}")]
struct PasswordExample {
    policy: PasswordPolicy,
    example: String,
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

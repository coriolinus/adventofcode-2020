use aoc2020::CommaSep;

use std::{convert::TryFrom, path::Path, str::FromStr};
use thiserror::Error;

#[derive(Clone, parse_display::FromStr, parse_display::Display)]
#[display("{name}: {low_range_low}-{low_range_high} or {high_range_low}-{high_range_high}")]
struct TicketField {
    name: String,
    low_range_low: u32,
    low_range_high: u32,
    high_range_low: u32,
    high_range_high: u32,
}

#[derive(Clone, Default)]
struct Input {
    fields: Vec<TicketField>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = Input::default();

        for (section_idx, section) in s.split("\n\n").enumerate() {
            match section_idx {
                0 => {
                    // field descriptors
                    for line in section.lines() {
                        input.fields.push(line.trim().parse()?);
                    }
                }
                1 => {
                    // my ticket
                    const INITIAL: &str = "your ticket:\n";
                    if !section.starts_with(INITIAL) {
                        return Err(Error::MissingInitial(section_idx));
                    }
                    let section = &section[INITIAL.len()..];
                    let fields: CommaSep<u32> = section.trim().parse()?;
                    input.my_ticket = fields.into_iter().collect();
                }
                2 => {
                    // nearby tickets
                    const INITIAL: &str = "nearby tickets:\n";
                    if !section.starts_with(INITIAL) {
                        return Err(Error::MissingInitial(section_idx));
                    }
                    let section = &section[INITIAL.len()..];

                    for line in section.lines() {
                        let fields: CommaSep<u32> = line.trim().parse()?;
                        input.nearby_tickets.push(fields.into_iter().collect());
                    }
                }
                _ => return Err(Error::TooManySections),
            }
        }

        Ok(input)
    }
}

impl TryFrom<&Path> for Input {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let data = std::fs::read_to_string(path)?;
        Input::from_str(&data)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let input = Input::try_from(input)?;
    println!("input parsed successfully");
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parsing ticket field")]
    Field(#[from] parse_display::ParseError),
    #[error("parsing number")]
    Int(#[from] std::num::ParseIntError),
    #[error("input file had too many sections")]
    TooManySections,
    #[error("section \"{0}\" missing its initializer")]
    MissingInitial(usize),
}

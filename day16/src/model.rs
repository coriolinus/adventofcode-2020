use super::Error;
use aoc2020::CommaSep;

use std::{convert::TryFrom, path::Path, str::FromStr};

#[derive(Clone, parse_display::FromStr, parse_display::Display)]
#[display("{name}: {low_range_low}-{low_range_high} or {high_range_low}-{high_range_high}")]
pub struct TicketField {
    pub(crate) name: String,
    low_range_low: u32,
    low_range_high: u32,
    high_range_low: u32,
    high_range_high: u32,
}

impl TicketField {
    pub fn contains(&self, n: u32) -> bool {
        (self.low_range_low..=self.low_range_high).contains(&n)
            || (self.high_range_low..=self.high_range_high).contains(&n)
    }
}

pub type Ticket = Vec<u32>;

#[derive(Clone, Default)]
pub struct Input {
    pub(crate) fields: Vec<TicketField>,
    pub(crate) my_ticket: Ticket,
    pub(crate) nearby_tickets: Vec<Ticket>,
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

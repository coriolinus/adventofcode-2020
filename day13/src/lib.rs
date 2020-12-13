use aoc2020::{
    input::parse_newline_sep,
    numbers::chinese_remainder::{chinese_remainder, Constraint},
};

use std::{path::Path, str::FromStr};
use thiserror::Error;

type Bus = i64;
type Timestamp = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
enum BusId {
    #[display("{0}")]
    Number(Bus),
    #[display("x")]
    X,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct BusNotes {
    earliest_departure_timestamp: Timestamp,
    routes: Vec<BusId>,
}

impl FromStr for BusNotes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut notes = BusNotes::default();
        for (idx, mut line) in s.split('\n').enumerate() {
            line = line.trim();
            match idx {
                0 => notes.earliest_departure_timestamp = line.parse()?,
                1 => {
                    for bus in line.split(',') {
                        notes.routes.push(bus.parse()?);
                    }
                }
                _ => {
                    if !line.is_empty() {
                        return Err(Error::TooManyLines);
                    }
                }
            }
        }
        Ok(notes)
    }
}

fn minutes_remaining_after(bus: Bus, t: Timestamp) -> Timestamp {
    let point_in_route = t % bus;
    if point_in_route == 0 {
        0
    } else {
        bus - point_in_route
    }
}

impl BusNotes {
    fn active_routes(&self) -> impl '_ + Iterator<Item = (usize, Bus)> {
        self.routes
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(idx, bus)| match bus {
                BusId::Number(bus) => Some((idx, bus)),
                BusId::X => None,
            })
    }

    fn first_departure_after(&self) -> Option<(Timestamp, Bus)> {
        self.active_routes()
            .map(|(_, bus)| {
                let minutes_remaining =
                    minutes_remaining_after(bus, self.earliest_departure_timestamp);
                (minutes_remaining, bus)
            })
            .min()
    }

    fn is_valid_part2(&self, t: Timestamp) -> bool {
        self.routes
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(idx, bus)| match bus {
                BusId::Number(bus) => Some((idx, bus)),
                BusId::X => None,
            })
            .all(|(idx, bus)| minutes_remaining_after(bus, t) as usize == idx)
    }

    fn search_for_valid_timestamp(&self) -> Option<Timestamp> {
        let constraints: Vec<_> = self
            .active_routes()
            .map(|(position, bus)| Constraint::new_invert_remainder(bus, position as Bus))
            .collect();
        let t = chinese_remainder(&constraints)?;
        let valid = self.is_valid_part2(t);
        if !valid {
            dbg!(t);
        }
        assert!(valid, "validity calculation failed");
        Some(t)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for (notes_id, notes) in parse_newline_sep::<BusNotes>(input)?.enumerate() {
        let (minutes_remaining, bus) = notes.first_departure_after().ok_or(Error::NoSolution)?;
        println!(
            "notes {}: id * remaining_time = {}",
            notes_id,
            minutes_remaining * bus
        );
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for (notes_id, notes) in parse_newline_sep::<BusNotes>(input)?.enumerate() {
        let valid_timestamp = notes
            .search_for_valid_timestamp()
            .ok_or(Error::NoSolution)?;
        println!(
            "notes {}: first valid timestamp = {}",
            notes_id, valid_timestamp
        );
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Int(#[from] std::num::ParseIntError),
    #[error(transparent)]
    Id(#[from] parse_display::ParseError),
    #[error("too many lines of notes")]
    TooManyLines,
    #[error("no solution found")]
    NoSolution,
}

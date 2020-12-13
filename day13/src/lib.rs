use aoc2020::input::parse_newline_sep;

use std::{path::Path, str::FromStr};
use thiserror::Error;

type Bus = u32;
type Timestamp = u32;

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

impl BusNotes {
    fn active_routes(&self) -> impl '_ + Iterator<Item = Bus> {
        self.routes.iter().copied().filter_map(|id| match id {
            BusId::Number(bus) => Some(bus),
            BusId::X => None,
        })
    }

    fn first_departure_after(&self) -> Option<(Timestamp, Bus)> {
        self.active_routes()
            .map(|bus| {
                let point_in_route = self.earliest_departure_timestamp % bus;
                let minutes_remaining = if point_in_route == 0 {
                    0
                } else {
                    bus - point_in_route
                };
                (minutes_remaining, bus)
            })
            .min()
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

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
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

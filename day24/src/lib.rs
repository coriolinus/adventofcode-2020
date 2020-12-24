use aoc2020::parse;
use std::{
    ops::{Add, AddAssign},
    path::Path,
    str::FromStr,
};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HexDirection {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl HexDirection {
    fn all() -> [HexDirection; 6] {
        [
            HexDirection::East,
            HexDirection::Southeast,
            HexDirection::Southwest,
            HexDirection::West,
            HexDirection::Northwest,
            HexDirection::Northeast,
        ]
    }

    fn try_parse(s: &str) -> (Option<HexDirection>, &str) {
        let mut chars = s.chars();
        let first = chars.next();
        let second = chars.next();
        match (first, second) {
            (Some('e'), _) => (Some(HexDirection::East), &s[1..]),
            (Some('s'), Some('e')) => (Some(HexDirection::Southeast), &s[2..]),
            (Some('s'), Some('w')) => (Some(HexDirection::Southwest), &s[2..]),
            (Some('w'), _) => (Some(HexDirection::West), &s[1..]),
            (Some('n'), Some('w')) => (Some(HexDirection::Northwest), &s[2..]),
            (Some('n'), Some('e')) => (Some(HexDirection::Northeast), &s[2..]),
            _ => (None, s),
        }
    }
}

struct HexDirections(Vec<HexDirection>);

impl FromStr for HexDirections {
    type Err = Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut directions = Vec::with_capacity(s.len());

        while !s.is_empty() {
            let (direction, remaining) = HexDirection::try_parse(s);
            match direction {
                None => return Err(Error::ParseFailure),
                Some(direction) => directions.push(direction),
            }

            s = remaining;
        }

        Ok(HexDirections(directions))
    }
}

/// Axial hex coordinates.
///
/// See [reference](https://www.redblobgames.com/grids/hexagons/#coordinates).
///
/// Constraint: `q + r + s == 0`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HexCoordinate {
    q: i32,
    r: i32,
}

impl HexCoordinate {
    #[inline]
    fn s(&self) -> i32 {
        -self.q - self.r
    }
}

impl AddAssign<HexDirection> for HexCoordinate {
    fn add_assign(&mut self, rhs: HexDirection) {
        match rhs {
            HexDirection::East => {
                self.q += 1;
            }
            HexDirection::Southeast => {
                self.r += 1;
            }
            HexDirection::Southwest => {
                self.q -= 1;
                self.r += 1;
            }
            HexDirection::West => {
                self.q -= 1;
            }
            HexDirection::Northwest => {
                self.r -= 1;
            }
            HexDirection::Northeast => {
                self.q += 1;
                self.r -= 1;
            }
        }
    }
}

impl Add<HexDirection> for HexCoordinate {
    type Output = HexCoordinate;

    fn add(mut self, rhs: HexDirection) -> Self::Output {
        self += rhs;
        self
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let directions: Vec<_> = parse::<HexDirections>(input)?.map(|hd| hd.0).collect();
    println!("parsed {} directions", directions.len());
    unimplemented!()
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parse failure")]
    ParseFailure,
}

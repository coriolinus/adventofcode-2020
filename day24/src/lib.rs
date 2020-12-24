use aoc2020::parse;
use std::{
    collections::HashSet,
    iter::FromIterator,
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
    fn iter() -> impl Iterator<Item = HexDirection> {
        std::iter::successors(Some(HexDirection::East), |direction| {
            use HexDirection::*;

            match direction {
                East => Some(Southeast),
                Southeast => Some(Southwest),
                Southwest => Some(West),
                West => Some(Northwest),
                Northwest => Some(Northeast),
                Northeast => None,
            }
        })
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
struct HexCoordinate {
    q: i32,
    r: i32,
}

impl HexCoordinate {
    #[inline]
    fn s(&self) -> i32 {
        -self.q - self.r
    }

    fn neighbors(self) -> impl 'static + Iterator<Item = HexCoordinate> {
        HexDirection::iter().map(move |direction| self + direction)
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

#[derive(Debug, Default, Clone)]
struct HexMap {
    coords: HashSet<HexCoordinate>,
}

impl HexMap {
    fn toggle(&mut self, coord: HexCoordinate) {
        // try to remove the coordinate.
        // `remove` returns whether the value was present, so if it wasn't,
        // then we can add it.
        if !self.coords.remove(&coord) {
            self.coords.insert(coord);
        }
    }

    fn is_black(&self, coord: HexCoordinate) -> bool {
        self.coords.contains(&coord)
    }

    fn count_black_neighbors_of(&self, coord: HexCoordinate) -> usize {
        coord
            .neighbors()
            .filter(|coord| self.coords.contains(coord))
            .count()
    }

    fn checked_coordinates(&self) -> HashSet<HexCoordinate> {
        let mut checked = self.coords.clone();
        for coord in self.coords.iter() {
            for neighbor in coord.neighbors() {
                checked.insert(neighbor);
            }
        }
        checked
    }

    fn conway_step(&self) -> HexMap {
        let mut successor = self.clone();

        for coord in self.checked_coordinates() {
            match (self.is_black(coord), self.count_black_neighbors_of(coord)) {
                (true, n) if n == 0 || n > 2 => {
                    successor.coords.remove(&coord);
                }
                (false, 2) => {
                    successor.coords.insert(coord);
                }
                _ => {
                    // leave all other tiles as they are
                }
            }
        }

        successor
    }
}

impl FromIterator<HexDirections> for HexMap {
    fn from_iter<T: IntoIterator<Item = HexDirections>>(iter: T) -> Self {
        let mut map = HexMap::default();

        for HexDirections(directions) in iter.into_iter() {
            let mut coord = HexCoordinate::default();
            for direction in directions {
                coord += direction;
            }
            map.toggle(coord);
        }

        map
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let map: HexMap = parse(input)?.collect();
    println!("black tiles: {}", map.coords.len());
    Ok(())
}

pub fn part2(input: &Path, trace: bool) -> Result<(), Error> {
    let mut map: HexMap = parse(input)?.collect();
    for i in 1..=100 {
        map = map.conway_step();
        if trace && (i < 10 || i % 10 == 0) {
            println!("Day {:3}: {}", i, map.coords.len());
        }
    }
    println!("black tiles (Day 100): {}", map.coords.len());
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parse failure")]
    ParseFailure,
}

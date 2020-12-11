use aoc2020::geometry::{Map, Point};

use std::{convert::TryFrom, path::Path};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, parse_display::Display)]
enum Tile {
    #[display(".")]
    Floor,
    #[display("L")]
    EmptySeat,
    #[display("#")]
    OccupiedSeat,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::EmptySeat),
            '#' => Ok(Self::OccupiedSeat),
            other => Err(format!("unexpected tile char: '{}'", other)),
        }
    }
}

type SeatingSystem = Map<Tile>;

fn count_occupied_adjacencies(seats: &SeatingSystem, position: Point) -> usize {
    seats
        .adjacencies(position)
        .filter(|&seat_position| seats[seat_position] == Tile::OccupiedSeat)
        .count()
}

fn count_occupied_projected(seats: &SeatingSystem, position: Point) -> usize {
    seats
        .adjacencies(position)
        .filter(|&adj| {
            let deltas = adj - position;
            assert!(!(deltas.x == 0 && deltas.y == 0));
            for visible_position in seats.project(position, deltas.x, deltas.y).skip(1) {
                match seats[visible_position] {
                    Tile::EmptySeat => return false,
                    Tile::OccupiedSeat => return true,
                    Tile::Floor => {}
                }
            }
            false
        })
        .count()
}

// panics if `seats` and `successor` have unequal bounds
fn state_transition(
    seats: &SeatingSystem,
    successor: &mut SeatingSystem,
    count_occupied: impl Fn(&SeatingSystem, Point) -> usize,
    max_adjacent: usize,
) {
    successor.for_each_point_mut(|seat, position| {
        let n_occupied_adjacencies = count_occupied(seats, position);
        match (&seat, n_occupied_adjacencies) {
            (Tile::EmptySeat, 0) => *seat = Tile::OccupiedSeat,
            (Tile::OccupiedSeat, n) if n >= max_adjacent => *seat = Tile::EmptySeat,
            _ => *seat = seats[position],
        }
    });
}

fn state_transition_adjacent(seats: &SeatingSystem, successor: &mut SeatingSystem) {
    state_transition(seats, successor, count_occupied_adjacencies, 4)
}

fn state_transition_project(seats: &SeatingSystem, successor: &mut SeatingSystem) {
    state_transition(seats, successor, count_occupied_projected, 5)
}

fn transition_until_stable(
    seats: &SeatingSystem,
    state_transition: impl Fn(&SeatingSystem, &mut SeatingSystem),
) -> SeatingSystem {
    let mut current = seats.clone();
    let mut successor = seats.clone();

    loop {
        state_transition(&current, &mut successor);
        if successor == current {
            break;
        }
        std::mem::swap(&mut current, &mut successor);
    }

    current
}

fn count_occupied(seats: &SeatingSystem) -> usize {
    seats
        .iter()
        .filter(|&seat| *seat == Tile::OccupiedSeat)
        .count()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let seats = SeatingSystem::try_from(input)?;
    let seats = transition_until_stable(&seats, state_transition_adjacent);
    let occupied_when_stable = count_occupied(&seats);
    println!(
        "seats occupied in steady state (adjacent):  {}",
        occupied_when_stable
    );
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let seats = SeatingSystem::try_from(input)?;
    let seats = transition_until_stable(&seats, state_transition_project);
    let occupied_when_stable = count_occupied(&seats);
    println!(
        "seats occupied in steady state (projected): {}",
        occupied_when_stable
    );
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    fn example() -> SeatingSystem {
        SeatingSystem::try_from(EXAMPLE.trim()).unwrap()
    }

    #[test]
    fn transitions() {
        let mut current = example();
        let mut n = 0;
        while n < 7 {
            n += 1;
            println!("{}", current);
            current = state_transition_project(&current);
        }
    }
}

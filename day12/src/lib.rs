use aoc2020::{
    geometry::{line_segment::LineSegment as Vector, Direction, Point},
    parse,
};

use std::path::Path;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, parse_display::FromStr, parse_display::Display)]
enum Action {
    #[display("N")]
    North,
    #[display("S")]
    South,
    #[display("E")]
    East,
    #[display("W")]
    West,
    #[display("L")]
    Left,
    #[display("R")]
    Right,
    #[display("F")]
    Forward,
}

#[derive(Clone, Copy, PartialEq, Eq, parse_display::FromStr, parse_display::Display)]
#[display("{action}{distance}")]
#[from_str(regex = r"(?P<action>\w)(?P<distance>\d+)")]
struct Instruction {
    action: Action,
    distance: i32,
}

#[derive(Clone, PartialEq, Eq)]
struct Ship {
    heading: Direction,
    position: Point,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            heading: Direction::Right,
            position: Point::default(),
        }
    }

    fn apply(&mut self, instruction: Instruction) {
        let movement_direction = match instruction.action {
            Action::North => Some(Direction::Up),
            Action::South => Some(Direction::Down),
            Action::East => Some(Direction::Right),
            Action::West => Some(Direction::Left),
            Action::Forward => Some(self.heading),
            Action::Left => {
                for _ in 0..(instruction.distance / 90) {
                    self.heading = self.heading.turn_left();
                }
                None
            }
            Action::Right => {
                for _ in 0..(instruction.distance / 90) {
                    self.heading = self.heading.turn_right();
                }
                None
            }
        };

        if let Some(direction) = movement_direction {
            let vector = Vector {
                direction,
                distance: instruction.distance,
            };
            self.position += vector;
        }
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut ship = Ship::new();
    for instruction in parse::<Instruction>(input)? {
        ship.apply(instruction);
    }
    let manhattan = ship.position.manhattan();
    println!("ship manhattan distance from origin: {}", manhattan);
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

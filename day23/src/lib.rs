use aoc2020::parse;

use std::{collections::VecDeque, fmt, path::Path, str::FromStr};
use thiserror::Error;

#[derive(Clone, Default)]
struct CupGame {
    cups: VecDeque<u32>,
    max: u32,
}

impl FromStr for CupGame {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = CupGame::default();
        for ch in s.chars() {
            game.cups.push_back(ch.to_string().parse()?);
        }
        if game.cups.len() < 5 {
            return Err(Error::TooFewCups);
        }
        game.max = game.cups.iter().copied().max().unwrap_or_default();
        Ok(game)
    }
}

impl CupGame {
    fn turn(&mut self) {
        let current = self.cups[0];
        self.cups.rotate_left(1);
        let mut picked_up: Vec<_> = self.cups.drain(..3).collect();

        let mut destination = current - 1;
        if destination == 0 {
            destination = self.max;
        }
        while picked_up.contains(&destination) {
            destination -= 1;
            if destination == 0 {
                destination = self.max;
            }
        }

        while self.cups[0] != destination {
            self.cups.rotate_left(1);
        }
        self.cups.rotate_left(1);
        // destination is at the end

        self.cups.extend(picked_up.drain(..));

        while self.cups[0] != current {
            self.cups.rotate_left(1);
        }
        self.cups.rotate_left(1);
    }

    fn extend_to(&mut self, n: u32) {
        self.cups.extend((self.max + 1)..=n);
        self.max = n;
    }

    fn rotate_to_1(&mut self) {
        while self.cups[0] != 1 {
            self.cups.rotate_left(1);
        }
    }
}

impl fmt::Display for CupGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut game: CupGame = self.clone();
        game.rotate_to_1();
        for n in game.cups.iter().skip(1) {
            write!(f, "{}", n)?;
        }
        Ok(())
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for (idx, mut game) in parse::<CupGame>(input)?.enumerate() {
        for _ in 0..100 {
            game.turn();
        }
        println!("input line {}: state after 100 moves: {}", idx, game);
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for (idx, mut game) in parse::<CupGame>(input)?.enumerate() {
        game.extend_to(1_000_000);
        for _ in 0..10_000_000 {
            game.turn();
        }
        game.rotate_to_1();
        let product = game.cups[1] as u64 * game.cups[2] as u64;
        println!(
            "input line {}: product of first 2 after 1 after ten million moves: {}",
            idx, product
        );
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Num(#[from] std::num::ParseIntError),
    #[error("the game doesn't work without at least 5 cups")]
    TooFewCups,
}

use aoc2020::parse;

use std::{collections::HashMap, fmt, path::Path, str::FromStr};
use thiserror::Error;

#[derive(Clone, Default)]
struct CupGame {
    successors: HashMap<u32, u32>,
    max: u32,
    current: u32,
}

impl FromStr for CupGame {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut first = None;
        let mut predecessor = 0;
        let mut n = 0;
        let mut game = CupGame::default();

        for ch in s.chars() {
            n = ch.to_string().parse()?;
            if first.is_none() {
                first = Some(n);
            }
            game.successors.insert(predecessor, n);
            predecessor = n;
        }
        game.successors.remove(&0);

        if game.successors.len() < 5 {
            return Err(Error::TooFewCups);
        }

        // we know that these must have been set, so just unwrap
        game.successors.insert(n, first.unwrap());
        game.max = game.successors.keys().copied().max().unwrap();
        game.current = first.unwrap();

        Ok(game)
    }
}

impl CupGame {
    fn turn(&mut self, trace: bool) {
        let mut stash = [0; 3];
        stash[0] = self.successors[&self.current];
        stash[1] = self.successors[&stash[0]];
        stash[2] = self.successors[&stash[1]];
        let subsequent = self.successors[&stash[2]];

        if trace {
            print!("cups: ({}) ", self.current);
            let mut next = self.successors[&self.current];
            while next != self.current {
                print!("{} ", next);
                next = self.successors[&next];
            }
            println!();

            println!("pick up: {:?}", stash);
        }

        // excise the cup stash from the cycle
        self.successors.insert(self.current, subsequent);

        // pick destination
        let mut destination = self.current - 1;
        let mut validated_destination = false;
        while !validated_destination {
            if destination == 0 {
                destination = self.max;
            }
            if stash.contains(&destination) {
                destination -= 1;
            } else {
                validated_destination = true;
            }
        }

        if trace {
            println!("destination: {}", destination);
        }

        // place stash after destination
        let after_destination = self.successors[&destination];
        self.successors.insert(stash[2], after_destination);
        self.successors.insert(destination, stash[0]);

        // update current
        self.current = self.successors[&self.current];
    }

    fn extend_to(&mut self, n: u32) {
        unimplemented!()
    }
}

impl fmt::Display for CupGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut next = self.successors[&1];
        while next != 1 {
            write!(f, "{}", next)?;
            next = self.successors[&next];
        }
        Ok(())
    }
}

pub fn part1(input: &Path, trace: bool) -> Result<(), Error> {
    for (idx, mut game) in parse::<CupGame>(input)?.enumerate() {
        for i in 0..100 {
            if trace {
                println!("\n-- move {} --", i + 1);
            }
            game.turn(trace);
        }
        println!("input line {}: state after 100 moves: {}", idx, game);
    }
    Ok(())
}

pub fn part2(input: &Path, trace: bool) -> Result<(), Error> {
    for (idx, mut game) in parse::<CupGame>(input)?.enumerate() {
        game.extend_to(1_000_000);
        for _ in 0..10_000_000 {
            game.turn(trace);
        }
        let first_successor = game.successors[&1];
        let second_successor = game.successors[&first_successor];
        let product = first_successor as u64 * second_successor as u64;
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

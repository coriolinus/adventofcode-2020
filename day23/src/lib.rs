use aoc2020::parse;

use std::{fmt, path::Path, str::FromStr};
use thiserror::Error;

#[derive(Clone, Default)]
struct CupGame {
    successors: Vec<usize>,
    max: usize,
    current: usize,
}

impl FromStr for CupGame {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut first = None;
        let mut predecessor = 0;
        let mut n = 0;
        let mut game = CupGame::default();
        game.successors.resize(s.chars().count() + 1, 0);

        for ch in s.chars() {
            n = ch.to_string().parse()?;
            if first.is_none() {
                first = Some(n);
            }
            game.successors[predecessor] = n;
            predecessor = n;
        }
        // we both insert and remove one element after this point,
        // so we can check the length directly, even though the map
        // isn't precisely as it will be on a successful return
        if game.successors.len() < 5 {
            return Err(Error::TooFewCups);
        }

        // we know that these must have been set, so just unwrap
        game.successors[n] = first.unwrap();
        game.max = game.successors.iter().copied().max().unwrap();
        game.current = first.unwrap();

        Ok(game)
    }
}

impl CupGame {
    fn turn(&mut self, trace: bool) {
        let mut stash = [0; 3];
        stash[0] = self.successors[self.current];
        stash[1] = self.successors[stash[0]];
        stash[2] = self.successors[stash[1]];
        let subsequent = self.successors[stash[2]];

        if trace {
            print!("cups: ({}) ", self.current);
            let mut next = self.successors[self.current];
            while next != self.current {
                print!("{} ", next);
                next = self.successors[next];
            }
            println!();

            println!("pick up: {:?}", stash);
        }

        // excise the cup stash from the cycle
        self.successors[self.current] = subsequent;

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
        let after_destination = self.successors[destination];
        self.successors[stash[2]] = after_destination;
        self.successors[destination] = stash[0];

        // update current
        self.current = self.successors[self.current];
    }

    fn extend_to(&mut self, n: usize) {
        let (mut prev, _current) = self
            .successors
            .iter()
            .enumerate()
            .skip(1)
            .find(|(_prev, &succ)| succ == self.current)
            .unwrap();
        self.successors.resize(n + 1, 0);
        for i in (self.max + 1)..=n {
            self.successors[prev] = i;
            prev = i;
        }
        self.successors[n] = self.current;
        self.max = n;
    }
}

impl fmt::Display for CupGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut next = self.successors[1];
        while next != 1 {
            write!(f, "{}", next)?;
            next = self.successors[next];
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
        let first_successor = game.successors[1];
        let second_successor = game.successors[first_successor];
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_extend() {
        let mut game = CupGame::from_str("54321").unwrap();
        game.extend_to(10);
        assert_eq!(game.to_string(), "6789105432");
    }

    #[test]
    fn test_extend_count() {
        let mut game = CupGame::from_str("389125467").unwrap();
        game.extend_to(1_000_000);

        // test 1: it takes 1 million steps to cycle through the list
        let mut count = 1;
        let mut next = game.successors[1];
        while next != 1 {
            next = game.successors[next];
            count += 1;
        }

        assert_eq!(count, 1_000_000);

        // test 2: there are a million distinct entries in the list
        let mut set: HashSet<usize> = HashSet::with_capacity(game.successors.len());
        set.extend(game.successors.iter().skip(1));
        assert_eq!(set.len(), 1_000_000);

        // test 3: every number from 1 to a million is in the set
        for n in 1..=1_000_000 {
            assert!(set.contains(&n));
        }
    }

    #[test]
    fn test_example() {
        let mut game = CupGame::from_str("389125467").unwrap();
        for _ in 0..100 {
            game.turn(true);
        }
        assert_eq!(game.to_string(), "67384529");
    }

    #[test]
    fn test_part2_example() {
        let mut game = CupGame::from_str("389125467").unwrap();
        game.extend_to(1_000_000);
        for _ in 0..10_000_000 {
            game.turn(false);
        }
        let s1 = game.successors[1];
        let s2 = game.successors[s1];
        let product = s1 * s2;

        assert_eq!(product, 149245887792);
    }
}

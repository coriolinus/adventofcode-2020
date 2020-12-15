use aoc2020::{parse, CommaSep};

use std::{collections::HashMap, path::Path};
use thiserror::Error;

fn memory_game(initializers: &[u32]) -> impl '_ + Iterator<Item = u32> {
    let mut turn = 0;
    let mut previous = 0;
    let mut most_recent = HashMap::<u32, usize>::new();
    let mut second_most_recent = HashMap::<u32, usize>::new();

    std::iter::from_fn(move || {
        let current = if turn < initializers.len() {
            initializers[turn]
        } else {
            second_most_recent
                .get(&previous)
                .map(|&second_most| (turn - second_most) as u32)
                .unwrap_or_default()
        };
        turn += 1;
        if let Some(most_recent) = most_recent.get(&current) {
            second_most_recent.insert(current, *most_recent);
        }
        most_recent.insert(current, turn);
        previous = current;
        Some(current)
    })
}

pub fn part1(input: &Path) -> Result<(), Error> {
    const N: usize = 2020;
    let initializers: Vec<u32> = parse::<CommaSep<u32>>(input)?.flatten().collect();
    let value = memory_game(&initializers)
        .nth(N - 1)
        .expect("game never terminates; qed");
    println!("{}th number spoken: {}", N, value);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    const N: usize = 30000000;
    let initializers: Vec<u32> = parse::<CommaSep<u32>>(input)?.flatten().collect();
    let value = memory_game(&initializers)
        .nth(N - 1)
        .expect("game never terminates; qed");
    println!("{}th number spoken: {}", N, value);
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

    #[test]
    fn test_example() {
        let initializers = [0, 3, 6];
        let expect = [0_u32, 3, 6, 0, 3, 3, 1, 0, 4, 0];
        for (actual, expect) in memory_game(&initializers).zip(&expect) {
            assert_eq!(actual, *expect);
        }
    }

    #[test]
    fn test_nth_semantics() {
        // nth has these semantics: for Turn `N`, request `nth(N-1)`.
        let initializers = [0, 3, 6];
        assert_eq!(memory_game(&initializers).nth(0), Some(0));
        assert_eq!(memory_game(&initializers).nth(8), Some(4));
        assert_eq!(memory_game(&initializers).nth(2020 - 1), Some(436));
    }
}

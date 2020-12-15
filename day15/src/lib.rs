use aoc2020::{parse, CommaSep};

use std::path::Path;
use thiserror::Error;

fn memory_game(initializers: &[u32], preallocate: usize) -> impl '_ + Iterator<Item = u32> {
    let mut turn = 0;
    let mut previous = 0;
    let mut most_recent = Vec::with_capacity(preallocate);
    let mut second_most_recent = Vec::with_capacity(preallocate);

    std::iter::from_fn(move || {
        let current = if turn < initializers.len() {
            initializers[turn]
        } else {
            second_most_recent
                .get(previous as usize)
                .map(|&second_most| {
                    if second_most != 0 {
                        (turn - second_most) as u32
                    } else {
                        0
                    }
                })
                .unwrap_or_default()
        };
        turn += 1;
        let ucurrent = current as usize;
        if most_recent.len() <= ucurrent {
            most_recent.resize(ucurrent + 1, 0);
        }
        if most_recent[ucurrent] != 0 {
            if second_most_recent.len() <= ucurrent {
                second_most_recent.resize(ucurrent + 1, 0);
            }
            second_most_recent[ucurrent] = most_recent[ucurrent]
        }
        most_recent[ucurrent] = turn;
        previous = current;
        Some(current)
    })
}

pub fn part1(input: &Path) -> Result<(), Error> {
    const N: usize = 2020;
    let initializers: Vec<u32> = parse::<CommaSep<u32>>(input)?.flatten().collect();
    let value = memory_game(&initializers, N / 10)
        .nth(N - 1)
        .expect("game never terminates; qed");
    println!("{}th number spoken: {}", N, value);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    const N: usize = 30000000;
    let initializers: Vec<u32> = parse::<CommaSep<u32>>(input)?.flatten().collect();
    let value = memory_game(&initializers, N / 10)
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
            dbg!(actual, expect);
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

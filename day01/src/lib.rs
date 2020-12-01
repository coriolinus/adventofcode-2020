use aoc2020::parse;

use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

fn find_pair_summing_to(data: &HashSet<i64>, sum: i64) -> Option<(i64, i64)> {
    for datum in data {
        let want = sum - *datum;
        if data.contains(&want) {
            return Some((*datum, want));
        }
    }
    None
}

fn find_triple_summing_to(data: &HashSet<i64>, sum: i64) -> Option<(i64, i64, i64)> {
    for datum in data {
        let remainder = sum - *datum;
        if let Some((a, b)) = find_pair_summing_to(data, remainder) {
            if a != b && a != *datum && b != *datum {
                return Some((a, b, *datum));
            }
        }
    }
    None
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let inputs: HashSet<i64> = parse(input)?.collect();
    match find_pair_summing_to(&inputs, 2020) {
        Some((a, b)) => {
            println!("{} * {} == {}", a, b, a * b);
        }
        None => println!("pair not found"),
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let inputs: HashSet<i64> = parse(input)?.collect();
    match find_triple_summing_to(&inputs, 2020) {
        Some((a, b, c)) => {
            println!("{} * {} * {} == {}", a, b, c, a * b * c);
        }
        None => println!("triple not found"),
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

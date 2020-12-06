use aoc2020::input::parse_newline_sep;

use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

struct CustomsDeclarationForm(Vec<HashSet<char>>);

impl std::str::FromStr for CustomsDeclarationForm {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .split_whitespace()
            .map(|personal_answers| personal_answers.chars().collect())
            .collect();
        Ok(CustomsDeclarationForm(inner))
    }
}

impl CustomsDeclarationForm {
    fn union(&self) -> HashSet<char> {
        self.0.iter().fold(HashSet::new(), |mut acc, elem| {
            acc.extend(elem);
            acc
        })
    }

    fn intersection(&self) -> HashSet<char> {
        self.0
            .iter()
            .fold(None, |acc, elem| match acc {
                None => Some(elem.clone()),
                Some(acc) => Some(acc.intersection(elem).copied().collect()),
            })
            .unwrap_or_default()
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let sum_of_counts: usize = parse_newline_sep::<CustomsDeclarationForm>(input)?
        .map(|cdf| cdf.union().len())
        .sum();
    println!("sum of union counts: {}", sum_of_counts);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let sum_of_counts: usize = parse_newline_sep::<CustomsDeclarationForm>(input)?
        .map(|cdf| cdf.intersection().len())
        .sum();
    println!("sum of intersection counts: {}", sum_of_counts);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

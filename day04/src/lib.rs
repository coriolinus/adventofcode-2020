use aoc2020::input::parse_newline_sep;

use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

#[derive(Default)]
struct PassportRaw {
    data: HashMap<String, String>,
}

impl PassportRaw {
    fn is_northpole(&self) -> bool {
        let expect_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        expect_fields
            .iter()
            .all(|&field| self.data.contains_key(field))
    }

    fn is_valid(&self) -> bool {
        self.is_northpole() && self.data.contains_key("cid")
    }
}

impl FromStr for PassportRaw {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pr = PassportRaw::default();

        for field in s.split_whitespace() {
            let mut parts = field.split(':');
            let key = parts
                .next()
                .ok_or_else(|| format!("missing key in '{}'", field))?;
            let value = parts
                .next()
                .ok_or_else(|| format!("missing value in '{}'", field))?;
            if parts.next().is_some() {
                Err(format!("too many parts in '{}'", field))?;
            }

            pr.data.insert(key.to_string(), value.to_string());
        }

        Ok(pr)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let valid = parse_newline_sep::<PassportRaw>(input)?
        .filter(|pr| pr.is_northpole())
        .count();
    println!("valid (northpole): {}", valid);
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

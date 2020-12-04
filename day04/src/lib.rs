use aoc2020::input::parse_newline_sep;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref HAIR_COLOR_RE: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    static ref EYE_COLOR_RE: Regex = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new(r"\d{9}").unwrap();
}

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

    fn parse_numeric_field(&self, field: &str, min: u32, max: u32) -> bool {
        let field_data = match self.data.get(field) {
            Some(field) => field,
            None => return false,
        };

        let field_num = match field_data.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return false,
        };

        field_num >= min && field_num <= max
    }

    fn is_valid_height(&self) -> bool {
        let hgt = match self.data.get("hgt") {
            Some(field) => field,
            None => return false,
        };
        let hgt: Height = match hgt.parse() {
            Ok(hgt) => hgt,
            Err(_) => return false,
        };

        match hgt {
            Height::Cm(cm) => cm >= 150 && cm <= 193,
            Height::In(inch) => inch >= 59 && inch <= 76,
        }
    }

    fn is_valid_re(&self, field: &str, re: &Regex) -> bool {
        let field = match self.data.get(field) {
            Some(field) => field,
            None => return false,
        };

        re.is_match(field)
    }

    fn is_valid(&self) -> bool {
        self.parse_numeric_field("byr", 1920, 2002)
            && self.parse_numeric_field("iyr", 2010, 2020)
            && self.parse_numeric_field("eyr", 2020, 2030)
            && self.is_valid_height()
            && self.is_valid_re("hcl", &HAIR_COLOR_RE)
            && self.is_valid_re("ecl", &EYE_COLOR_RE)
            && self.is_valid_re("pid", &PASSPORT_ID_RE)
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

enum Height {
    Cm(u32),
    In(u32),
}

impl FromStr for Height {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("cm") {
            let n = s[..s.len() - 2].parse::<u32>().map_err(|e| e.to_string())?;
            Ok(Self::Cm(n))
        } else if s.ends_with("in") {
            let n = s[..s.len() - 2].parse::<u32>().map_err(|e| e.to_string())?;
            Ok(Self::In(n))
        } else {
            Err("unrecognized measurement".into())
        }
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let valid = parse_newline_sep::<PassportRaw>(input)?
        .filter(|pr| pr.is_northpole())
        .count();
    println!("count (northpole): {}", valid);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let valid = parse_newline_sep::<PassportRaw>(input)?
        .filter(|pr| pr.is_valid())
        .count();
    println!("valid: {}", valid);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

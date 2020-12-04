use aoc2020::input::parse_newline_sep;

use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref HAIR_COLOR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref EYE_COLOR_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

#[derive(Default)]
struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt_set: bool,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn has_northpole_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt_set
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_inner(&self) -> Option<bool> {
        let valid = (1920..=2002).contains(&self.byr?)
            && (2010..=2020).contains(&self.iyr?)
            && (2020..=2030).contains(&self.eyr?)
            && match self.hgt.as_ref()? {
                Height::Cm(cm) => (150..=193).contains(cm),
                Height::In(inch) => (59..=76).contains(inch),
            }
            && HAIR_COLOR_RE.is_match(self.hcl.as_ref()?)
            && EYE_COLOR_RE.is_match(self.ecl.as_ref()?)
            && PASSPORT_ID_RE.is_match(self.pid.as_ref()?);
        Some(valid)
    }

    fn is_valid(&self) -> bool {
        self.is_valid_inner().unwrap_or_default()
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::default();

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

            match key {
                "byr" => passport.byr = Some(value.parse::<u32>().map_err(|e| e.to_string())?),
                "iyr" => passport.iyr = Some(value.parse::<u32>().map_err(|e| e.to_string())?),
                "eyr" => passport.eyr = Some(value.parse::<u32>().map_err(|e| e.to_string())?),
                "hgt" => {
                    passport.hgt_set = true;
                    passport.hgt = value.parse().ok();
                }
                "hcl" => passport.hcl = Some(value.to_string()),
                "ecl" => passport.ecl = Some(value.to_string()),
                "pid" => passport.pid = Some(value.to_string()),
                _ => {
                    // don't care about extra fields
                }
            }
        }

        Ok(passport)
    }
}

#[derive(parse_display::FromStr, Debug)]
enum Height {
    #[display("{0}cm")]
    Cm(u32),
    #[display("{0}in")]
    In(u32),
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let valid = parse_newline_sep::<Passport>(input)?
        .filter(|passport| passport.has_northpole_fields())
        .count();
    println!("count (northpole): {}", valid);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let valid = parse_newline_sep::<Passport>(input)?
        .filter(|passport| passport.is_valid())
        .count();
    println!("valid: {}", valid);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

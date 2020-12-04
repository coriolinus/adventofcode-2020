use aoc2020::input::parse_newline_sep;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref HAIR_COLOR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref EYE_COLOR_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

#[derive(Default)]
struct PassportRaw {
    data: HashMap<String, String>,
}

impl PassportRaw {
    fn has_northpole_fields(&self) -> bool {
        let expect_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        expect_fields
            .iter()
            .all(|&field| self.data.contains_key(field))
    }

    fn parse_numeric_field_inner(&self, field: &str, min: u32, max: u32) -> Option<bool> {
        let field = self.data.get(field)?;
        let field = field.parse::<u32>().ok()?;

        Some(field >= min && field <= max)
    }

    fn parse_numeric_field(&self, field: &str, min: u32, max: u32) -> bool {
        self.parse_numeric_field_inner(field, min, max)
            .unwrap_or_default()
    }

    fn is_valid_height_inner(&self) -> Option<bool> {
        let hgt = self.data.get("hgt")?;
        let hgt: Height = hgt.parse().ok()?;

        Some(match hgt {
            Height::Cm(cm) => cm >= 150 && cm <= 193,
            Height::In(inch) => inch >= 59 && inch <= 76,
        })
    }

    fn is_valid_height(&self) -> bool {
        self.is_valid_height_inner().unwrap_or_default()
    }

    fn is_valid_re_inner(&self, field: &str, re: &Regex) -> Option<bool> {
        let field = self.data.get(field)?;
        Some(re.is_match(field))
    }

    fn is_valid_re(&self, field: &str, re: &Regex) -> bool {
        self.is_valid_re_inner(field, re).unwrap_or_default()
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
        .filter(|pr| pr.has_northpole_fields())
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

pub fn invalidities(input: &Path) -> Result<(), Error> {
    let mut byr = HashSet::new();
    let mut iyr = HashSet::new();
    let mut eyr = HashSet::new();
    let mut hgt = HashSet::new();
    let mut hcl = HashSet::new();
    let mut ecl = HashSet::new();
    let mut pid = HashSet::new();

    for passport in parse_newline_sep::<PassportRaw>(input)? {
        if !passport.parse_numeric_field("byr", 1920, 2002) {
            if let Some(invalid) = passport.data.get("byr") {
                byr.insert(invalid.clone());
            }
        }
        if !passport.parse_numeric_field("iyr", 2010, 2020) {
            if let Some(invalid) = passport.data.get("iyr") {
                iyr.insert(invalid.clone());
            }
        }
        if !passport.parse_numeric_field("eyr", 2020, 2030) {
            if let Some(invalid) = passport.data.get("eyr") {
                eyr.insert(invalid.clone());
            }
        }
        if !passport.is_valid_height() {
            if let Some(invalid) = passport.data.get("hgt") {
                hgt.insert(invalid.clone());
            }
        }
        if !passport.is_valid_re("hcl", &HAIR_COLOR_RE) {
            if let Some(invalid) = passport.data.get("hcl") {
                hcl.insert(invalid.clone());
            }
        }
        if !passport.is_valid_re("ecl", &EYE_COLOR_RE) {
            if let Some(invalid) = passport.data.get("ecl") {
                ecl.insert(invalid.clone());
            }
        }
        if !passport.is_valid_re("pid", &PASSPORT_ID_RE) {
            if let Some(invalid) = passport.data.get("pid") {
                pid.insert(invalid.clone());
            }
        }
    }

    fn invalid(name: &str, set: HashSet<String>) {
        let mut v: Vec<_> = set.into_iter().collect();
        v.sort();
        println!("invalid {}:\n{:?}\n", name, v);
    }

    invalid("birth years", byr);
    invalid("issue years", iyr);
    invalid("expiration years", eyr);
    invalid("heights", hgt);
    invalid("hair colors", hcl);
    invalid("eye colors", ecl);
    invalid("passport ids", pid);

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

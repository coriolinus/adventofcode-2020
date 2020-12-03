use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn abs_sum(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

lazy_static! {
    static ref VEC3_RE: Regex = Regex::new(
        r"(?i)<\s*(x=\s*)?(?P<x>-?\d+),\s*(y=\s*)?(?P<y>-?\d+),\s*(z=\s*)?(?P<z>-?\d+)\s*>"
    )
    .unwrap();
}

impl FromStr for Vector3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = VEC3_RE.captures(s).ok_or(format!("no regex match"))?;
        Ok(Vector3 {
            x: captures
                .name("x")
                .unwrap()
                .as_str()
                .parse()
                .map_err(|err| format!("x: {}", err))?,
            y: captures
                .name("y")
                .unwrap()
                .as_str()
                .parse()
                .map_err(|err| format!("y: {}", err))?,
            z: captures
                .name("z")
                .unwrap()
                .as_str()
                .parse()
                .map_err(|err| format!("z: {}", err))?,
        })
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={:3}, y={:3}, z={:3}>", self.x, self.y, self.z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

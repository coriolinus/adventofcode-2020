use crate::geometry::Direction;
use std::convert::TryFrom;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    // on my machine, passing self by copy and reference are equally sized,
    // and passing by copy breaks the cleanest usage of this function in Iterator::map,
    // so I'm going to retain the reference behavior. I expect the compiler to
    // inline this function anyway.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn abs(&self) -> Point {
        Point {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(
            i32::try_from(x).unwrap_or(i32::MAX),
            i32::try_from(y).unwrap_or(i32::MAX),
        )
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(mut self, other: Point) -> Point {
        self += other;
        self
    }
}

impl AddAssign<(i32, i32)> for Point {
    fn add_assign(&mut self, (dx, dy): (i32, i32)) {
        self.x += dx;
        self.y += dy;
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(mut self, deltas: (i32, i32)) -> Point {
        self += deltas;
        self
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, direction: Direction) -> Point {
        self + direction.deltas()
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, other: i32) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

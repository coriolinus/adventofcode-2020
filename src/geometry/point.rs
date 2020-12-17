use crate::geometry::{line_segment::LineSegment, Direction};
use itertools::Itertools;
use std::{
    convert::TryFrom,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

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

    /// Rotate this point clockwise around the origin.
    ///
    /// For example:
    ///
    /// ```
    /// # use aoc2020::geometry::Point;
    /// let mut point = Point::new(2, 1);
    /// point = point.rotate_right();
    /// assert_eq!(point, Point::new(1, -2));
    /// point = point.rotate_right();
    /// assert_eq!(point, Point::new(-2, -1));
    /// point = point.rotate_right();
    /// assert_eq!(point, Point::new(-1, 2));
    /// point = point.rotate_right();
    /// assert_eq!(point, Point::new(2, 1));
    /// ```
    pub fn rotate_right(&self) -> Point {
        Point::new(self.y, -self.x)
    }

    /// Rotate this point counterclockwise around the origin.
    ///
    /// For example:
    ///
    /// ```
    /// # use aoc2020::geometry::Point;
    /// let mut point = Point::new(2, 1);
    /// point = point.rotate_left();
    /// assert_eq!(point, Point::new(-1, 2));
    /// point = point.rotate_left();
    /// assert_eq!(point, Point::new(-2, -1));
    /// point = point.rotate_left();
    /// assert_eq!(point, Point::new(1, -2));
    /// point = point.rotate_left();
    /// assert_eq!(point, Point::new(2, 1));
    /// ```
    pub fn rotate_left(&self) -> Point {
        Point::new(-self.y, self.x)
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
impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, direction: Direction) {
        *self += direction.deltas();
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(mut self, direction: Direction) -> Point {
        self += direction;
        self
    }
}

impl AddAssign<LineSegment> for Point {
    fn add_assign(
        &mut self,
        LineSegment {
            direction,
            distance,
        }: LineSegment,
    ) {
        let (mut dx, mut dy) = direction.deltas();
        dx *= distance;
        dy *= distance;
        *self += (dx, dy);
    }
}

impl Add<LineSegment> for Point {
    type Output = Point;

    fn add(mut self, line_segment: LineSegment) -> Point {
        self += line_segment;
        self
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

pub trait PointTrait: Copy + Eq {
    /// Numeric type backing this point
    type N;

    /// Return the manhattan distance of this point from the origin.
    fn manhattan(self) -> Self::N;

    /// Reduce all components of this point by 1.
    fn decr(self) -> Self;

    /// Increase all components of this point by 1.
    fn incr(self) -> Self;

    /// Generate all points inclusively bounded by `min` and `max`.
    fn inclusive_range(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>>;

    /// Iterate over points adjacent to this point.
    ///
    /// This includes diagonals, and excludes the center.
    ///
    /// The implementation should always return a constant number of items, even if
    /// for simplicity it does not implement `ExactSizeIterator`.
    fn adjacent(self) -> Box<dyn Iterator<Item = Self>>
    where
        Self: 'static,
    {
        Box::new(
            Self::inclusive_range(self.decr(), self.incr()).filter(move |&point| point != self),
        )
    }

    /// Return the boundary minimum between `self` and `other`.
    ///
    /// This is defined as a new point with each component defined by `self.component.min(other.component)`.
    fn boundary_min(self, other: Self) -> Self;

    /// Return the boundary maximum between `self` and `other`.
    ///
    /// This is defined as a new point with each component defined by `self.component.max(other.component)`.
    fn boundary_max(self, other: Self) -> Self;

    /// Return the volume of the space defined between this point and the origin.
    fn volume<T>(self) -> T
    where
        T: From<Self::N> + Mul<Output = T>;
}

impl PointTrait for Point {
    type N = i32;

    fn manhattan(self) -> Self::N {
        <Self>::manhattan(&self)
    }

    fn decr(self) -> Self {
        Point::new(self.x - 1, self.y - 1)
    }

    fn incr(self) -> Self {
        Point::new(self.x + 1, self.y + 1)
    }

    fn inclusive_range(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (min.y..=max.y)
                .cartesian_product(min.x..=max.x)
                .map(|(y, x)| Point::new(x, y)),
        )
    }

    fn boundary_min(self, other: Self) -> Self {
        Point::new(self.x.min(other.x), self.y.min(other.y))
    }

    fn boundary_max(self, other: Self) -> Self {
        Point::new(self.x.max(other.x), self.y.max(other.y))
    }

    fn volume<T>(self) -> T
    where
        T: From<Self::N> + Mul<Output = T>,
    {
        let x: T = self.x.abs().into();
        let y: T = self.y.abs().into();
        x * y
    }
}

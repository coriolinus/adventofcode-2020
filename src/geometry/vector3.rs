use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Return the manhattan distance of this vector from the origin
    pub fn abs_sum(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    /// Return this point with all dimensions decremented by 1
    pub fn decr(self) -> Vector3 {
        Vector3::new(self.x - 1, self.y - 1, self.z - 1)
    }

    /// Return this point with all dimensions incremented by 1
    pub fn incr(self) -> Vector3 {
        Vector3::new(self.x + 1, self.y + 1, self.z + 1)
    }

    /// Return all points that lie within the minimum and maximum bounds, inclusive
    pub fn inclusive_range(min: Vector3, max: Vector3) -> impl Iterator<Item = Vector3> {
        (min.x..=max.x)
            .cartesian_product(min.y..=max.y)
            .cartesian_product(min.z..=max.z)
            .map(|((x, y), z)| Vector3::new(x, y, z))
    }

    /// Iterate over points in 3d space adjacent to this point
    ///
    /// This includes diagonals, and excludes the center. It always returns 26 items.
    pub fn adjacent(self) -> impl Iterator<Item = Vector3> {
        Vector3::inclusive_range(self.decr(), self.incr()).filter(move |&v| v != self)
    }

    /// Return the boundary minimum between `self` and `other`.
    ///
    /// The standard `.min` function computes a total ordering between two vectors, but it doesn't
    /// help for computing an inclusive range. For example, it is true that
    ///
    /// ```rust
    /// # use aoc2020::geometry::vector3::Vector3;
    /// let a = Vector3::new(-1, -1, -1);
    /// let b = Vector3::new(0, -3, -1);
    /// assert!(a < b);
    /// ```
    ///
    /// The boundary minimum, on the other hand, computes the minimal bounded point which
    /// contains both `self` and `other`:
    ///
    /// ```rust
    /// # use aoc2020::geometry::vector3::Vector3;
    /// let a = Vector3::new(-1, -1, -1);
    /// let b = Vector3::new(0, -3, -1);
    /// assert_eq!(a.boundary_min(b), Vector3::new(-1, -3, -1));
    /// ```
    pub fn boundary_min(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    /// Return the boundary maximum between `self` and `other`.
    ///
    /// The standard `.max` function computes a total ordering between two vectors, but it doesn't
    /// help for computing an inclusive range. For example, it is true that
    ///
    /// ```rust
    /// # use aoc2020::geometry::vector3::Vector3;
    /// let a = Vector3::new(1, 1, 1);
    /// let b = Vector3::new(0, 3, 1);
    /// assert!(a > b);
    /// ```
    ///
    /// The boundary minimum, on the other hand, computes the minimal bounded point which
    /// contains both `self` and `other`:
    ///
    /// ```rust
    /// # use aoc2020::geometry::vector3::Vector3;
    /// let a = Vector3::new(1, 1, 1);
    /// let b = Vector3::new(0, 3, 1);
    /// assert_eq!(a.boundary_max(b), Vector3::new(1, 3, 1));
    /// ```
    pub fn boundary_max(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    /// Return the volume of the space defined between this point and the origin.
    pub fn volume<T>(self) -> T
    where
        T: From<i32> + Mul<Output = T>,
    {
        let x: T = self.x.abs().into();
        let y: T = self.y.abs().into();
        let z: T = self.z.abs().into();
        x * y * z
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

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(mut self, rhs: Vector3) -> Self::Output {
        self -= rhs;
        self
    }
}

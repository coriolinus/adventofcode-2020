use crate::geometry::point::PointTrait;
use itertools::Itertools;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// A point in 4-dimensional space
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl Vector4 {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Vector4 {
        Vector4 { x, y, z, w }
    }

    /// Return the manhattan distance of this vector from the origin
    pub fn abs_sum(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs() + self.w.abs()
    }

    /// Return this point with all dimensions decremented by 1
    pub fn decr(self) -> Vector4 {
        Vector4::new(self.x - 1, self.y - 1, self.z - 1, self.w - 1)
    }

    /// Return this point with all dimensions incremented by 1
    pub fn incr(self) -> Vector4 {
        Vector4::new(self.x + 1, self.y + 1, self.z + 1, self.w + 1)
    }

    /// Return all points that lie within the minimum and maximum bounds, inclusive
    pub fn inclusive_range(min: Vector4, max: Vector4) -> impl Iterator<Item = Vector4> {
        (min.x..=max.x)
            .cartesian_product(min.y..=max.y)
            .cartesian_product(min.z..=max.z)
            .cartesian_product(min.w..=max.w)
            .map(|(((x, y), z), w)| Vector4::new(x, y, z, w))
    }

    /// Iterate over points in 3d space adjacent to this point
    ///
    /// This includes diagonals, and excludes the center. It always returns 26 items.
    pub fn adjacent(self) -> impl Iterator<Item = Vector4> {
        Vector4::inclusive_range(self.decr(), self.incr()).filter(move |&v| v != self)
    }

    /// Return the boundary minimum between `self` and `other`.
    ///
    /// The standard `.min` function computes a total ordering between two vectors, but it doesn't
    /// help for computing an inclusive range. For example, it is true that
    ///
    /// ```rust
    /// # use aoc2020::geometry::vector3::Vector4;
    /// let a = Vector4::new(-1, -1, -1, 0);
    /// let b = Vector4::new(0, -3, -1, 0);
    /// assert!(a < b);
    /// ```
    ///
    /// The boundary minimum, on the other hand, computes the minimal bounded point which
    /// contains both `self` and `other`:
    ///
    /// ```rust
    /// # use aoc2020::geometry::vector3::Vector4;
    /// let a = Vector4::new(-1, -1, -1, 0);
    /// let b = Vector4::new(0, -3, -1, 0);
    /// assert_eq!(a.boundary_min(b), Vector4::new(-1, -3, -1, 0));
    /// ```
    pub fn boundary_min(self, other: Vector4) -> Vector4 {
        Vector4::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }

    /// Return the boundary maximum between `self` and `other`.
    ///
    /// The standard `.max` function computes a total ordering between two vectors, but it doesn't
    /// help for computing an inclusive range. For example, it is true that
    ///
    /// ```rust
    /// # use aoc2020::geometry::vector3::Vector4;
    /// let a = Vector4::new(1, 1, 1, 0);
    /// let b = Vector4::new(0, 3, 1, 0);
    /// assert!(a > b);
    /// ```
    ///
    /// The boundary minimum, on the other hand, computes the minimal bounded point which
    /// contains both `self` and `other`:
    ///
    /// ```rust
    /// # use aoc2020::geometry::vector3::Vector4;
    /// let a = Vector4::new(1, 1, 1, 0);
    /// let b = Vector4::new(0, 3, 1, 0);
    /// assert_eq!(a.boundary_max(b), Vector4::new(1, 3, 1, 0));
    /// ```
    pub fn boundary_max(self, other: Vector4) -> Vector4 {
        Vector4::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
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
        let w: T = self.w.abs().into();
        x * y * z * w
    }
}

impl AddAssign for Vector4 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Add for Vector4 {
    type Output = Vector4;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl SubAssign for Vector4 {
    fn sub_assign(&mut self, other: Vector4) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl Sub for Vector4 {
    type Output = Vector4;

    fn sub(mut self, rhs: Vector4) -> Self::Output {
        self -= rhs;
        self
    }
}

impl PointTrait for Vector4 {
    type N = i32;

    fn manhattan(self) -> Self::N {
        <Self>::abs_sum(self)
    }

    fn decr(self) -> Self {
        <Self>::decr(self)
    }

    fn incr(self) -> Self {
        <Self>::incr(self)
    }

    fn inclusive_range(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(<Self>::inclusive_range(min, max))
    }

    fn boundary_min(self, other: Self) -> Self {
        <Self>::boundary_min(self, other)
    }

    fn boundary_max(self, other: Self) -> Self {
        <Self>::boundary_max(self, other)
    }

    fn volume<T>(self) -> T
    where
        T: From<Self::N> + Mul<Output = T>,
    {
        <Self>::volume(self)
    }
}

use aoc2020::geometry::{
    point::{Point, PointTrait},
    tile::DisplayWidth,
    vector3::Vector3,
    vector4::Vector4,
    Map,
};

#[cfg(test)]
use aoc2020::geometry::tile::Bool;

use std::{collections::HashSet, convert::TryFrom, ops::Sub, path::Path};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::FromStr, parse_display::Display)]
enum Cube {
    #[display("#")]
    Active,
    #[display(".")]
    Inactive,
}

impl DisplayWidth for Cube {
    const DISPLAY_WIDTH: usize = 1;
}

#[derive(Default, Debug, Clone)]
pub struct ConwaySpace<HighDimensionPoint> {
    // choose a sparse representation instead of extending map because this space is specifically infinite
    active: HashSet<HighDimensionPoint>,
    min: HighDimensionPoint,
    max: HighDimensionPoint,
}

impl<HighDimensionPoint> ConwaySpace<HighDimensionPoint>
where
    HighDimensionPoint:
        'static + PointTrait + std::hash::Hash + Default + Sub<Output = HighDimensionPoint>,
    i64: From<<HighDimensionPoint as PointTrait>::N>,
{
    fn new<T, Projection>(
        input: T,
        projection: Projection,
    ) -> Result<ConwaySpace<HighDimensionPoint>, <Map<Cube> as TryFrom<T>>::Error>
    where
        Map<Cube>: TryFrom<T>,
        Projection: Fn(Point) -> HighDimensionPoint,
    {
        let plane = Map::try_from(input)?;
        let expected_capacity = plane.width() * plane.height();
        let mut space = ConwaySpace::with_capacity(expected_capacity);
        plane.for_each_point(|&cube, point| {
            let point = projection(point);
            if cube == Cube::Active {
                space.active.insert(point);
            }
            space.min = space.min.boundary_min(point);
            space.max = space.max.boundary_max(point);
        });
        Ok(space)
    }

    fn with_capacity(capacity: usize) -> ConwaySpace<HighDimensionPoint> {
        ConwaySpace {
            active: HashSet::with_capacity(capacity),
            ..ConwaySpace::default()
        }
    }

    fn get(&self, point: HighDimensionPoint) -> bool {
        self.active.contains(&point)
    }

    fn successor(&self) -> ConwaySpace<HighDimensionPoint> {
        let mut successor = ConwaySpace::default();

        for point in HighDimensionPoint::inclusive_range(self.min.decr(), self.max.incr()) {
            let n_adjacent = point.adjacent().filter(|&point| self.get(point)).count();
            match (self.get(point), n_adjacent) {
                (true, 2) | (true, 3) | (false, 3) => {
                    successor.active.insert(point);
                    successor.min = successor.min.boundary_min(point);
                    successor.max = successor.max.boundary_max(point);
                }
                _ => {
                    // in all other cases, the successor of this point is inactive
                }
            }
        }

        successor
    }

    fn nth_successor(&self, n: usize) -> ConwaySpace<HighDimensionPoint> {
        let mut successor = self.clone();

        for _ in 0..n {
            successor = successor.successor();
        }

        successor
    }

    /// extract a 2d plane from this conway space
    ///
    /// the projection takes an appropriate higher dimensional point and returns the tuple
    /// `(projected, on_plane)`.
    /// `projected` is the 2d projection of the point.
    /// `on_plane` is `true` when this point is on the plane of interest
    #[cfg(test)]
    fn plane_2d(&self, project: impl Fn(HighDimensionPoint) -> (Point, bool)) -> Map<Bool> {
        let (max, _) = project(self.max);
        let (min, _) = project(self.min);
        let width = max.x - min.x + 1;
        let height = max.y - min.y + 1;

        let mut plane = Map::new(width.abs() as usize, height.abs() as usize);

        for (point, on_plane) in self
            .active
            .iter()
            .map(|&point| point - self.min)
            .map(project)
        {
            if on_plane {
                plane[point] = true.into();
            }
        }

        plane
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    const N: usize = 6;
    let mut space = ConwaySpace::new(input, |point| Vector3::new(point.x, point.y, 0))?;
    space = space.nth_successor(N);
    let n_active = space.active.len();
    println!("{} active cubes (3d) after {} cycles", n_active, N);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    const N: usize = 6;
    let mut space = ConwaySpace::new(input, |point| Vector4::new(point.x, point.y, 0, 0))?;
    space = space.nth_successor(N);
    let n_active = space.active.len();
    println!("{} active cubes (4d) after {} cycles", n_active, N);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "
.#.
..#
###";

    const CYCLE_1A: &str = "
#..
..#
.#.";

    const CYCLE_1B: &str = "
#.#
.##
.#.";

    fn example<HighDimensionPoint>(
        projection: impl Fn(Point) -> HighDimensionPoint,
    ) -> ConwaySpace<HighDimensionPoint>
    where
        HighDimensionPoint:
            'static + PointTrait + std::hash::Hash + Default + Sub<Output = HighDimensionPoint>,
        i64: From<<HighDimensionPoint as PointTrait>::N>,
    {
        ConwaySpace::new(EXAMPLE.trim(), projection).unwrap()
    }

    fn check_projection<HighDimensionPoint>(
        space: &ConwaySpace<HighDimensionPoint>,
        projection: impl Fn(HighDimensionPoint) -> (Point, bool),
        expect: &str,
    ) where
        HighDimensionPoint:
            'static + PointTrait + std::hash::Hash + Default + Sub<Output = HighDimensionPoint>,
        i64: From<<HighDimensionPoint as PointTrait>::N>,
    {
        // we don't want to be faffing around matching indices; we just want to check that our
        // output matches our input. Just render it as a string, for simplicity.
        let expect_str = Map::<Bool>::try_from(expect.trim()).unwrap().to_string();
        let have = space.plane_2d(projection);
        let have_str = have.to_string();

        if have_str != expect_str {
            println!("have:\n{}", have_str);
            println!("expect:\n{}", expect_str);
        }

        assert_eq!(have.to_string(), expect_str);
    }

    #[test]
    fn test_projection() {
        let space = example(|point| Vector3::new(point.x, point.y, 0));
        check_projection(&space, |hd| (Point::new(hd.x, hd.y), hd.z == 0), EXAMPLE);
    }

    #[test]
    fn example_3_1() {
        let mut space = example(|point| Vector3::new(point.x, point.y, 0));
        space = space.successor();

        // z = -1
        check_projection(&space, |hd| (Point::new(hd.x, hd.y), hd.z == 0), CYCLE_1A);

        // z = 0
        check_projection(&space, |hd| (Point::new(hd.x, hd.y), hd.z == 1), CYCLE_1B);

        // z = 1
        check_projection(&space, |hd| (Point::new(hd.x, hd.y), hd.z == 2), CYCLE_1A);
    }

    #[test]
    fn example_4_1() {
        let mut space = example(|point| Vector4::new(point.x, point.y, 0, 0));
        space = space.successor();

        let z = -1;
        let w = -1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| (Point::new(hd.x, hd.y), hd.z == z + 1 && hd.w == w + 1),
            CYCLE_1A,
        );

        let z = 0;
        let w = -1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| (Point::new(hd.x, hd.y), hd.z == z + 1 && hd.w == w + 1),
            CYCLE_1A,
        );

        let z = 1;
        let w = -1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| (Point::new(hd.x, hd.y), hd.z == z + 1 && hd.w == w + 1),
            CYCLE_1A,
        );

        let z = -1;
        let w = 0;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| (Point::new(hd.x, hd.y), hd.z == z + 1 && hd.w == w + 1),
            CYCLE_1A,
        );

        let z = 0;
        let w = 0;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| (Point::new(hd.x, hd.y), hd.z == z + 1 && hd.w == w + 1),
            CYCLE_1B,
        );

        let z = 1;
        let w = 0;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| (Point::new(hd.x, hd.y), hd.z == z + 1 && hd.w == w + 1),
            CYCLE_1A,
        );

        let z = -1;
        let w = 1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| (Point::new(hd.x, hd.y), hd.z == z + 1 && hd.w == w + 1),
            CYCLE_1A,
        );

        let z = 0;
        let w = 1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| (Point::new(hd.x, hd.y), hd.z == z + 1 && hd.w == w + 1),
            CYCLE_1A,
        );

        let z = 1;
        let w = 1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| (Point::new(hd.x, hd.y), hd.z == z + 1 && hd.w == w + 1),
            CYCLE_1A,
        );
    }

    #[test]
    fn example_4_2() {
        let mut space = example(|point| Vector4::new(point.x, point.y, 0, 0));
        let prev = space.successor();
        space = prev.successor();

        const OFFSET: i32 = 2;

        let z = -2;
        let w = -2;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
..#..
.....
.....",
        );

        let z = -1;
        let w = -2;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 0;
        let w = -2;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
###..
##.##
#...#
.#..#
.###.",
        );

        let z = 1;
        let w = -2;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 2;
        let w = -2;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
..#..
.....
.....",
        );

        let z = -2;
        let w = -1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = -1;
        let w = -1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 0;
        let w = -1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 1;
        let w = -1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 2;
        let w = -1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = -2;
        let w = 0;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
###..
##.##
#...#
.#..#
.###.",
        );

        let z = -1;
        let w = 0;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 0;
        let w = 0;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 1;
        let w = 0;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 2;
        let w = 0;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
###..
##.##
#...#
.#..#
.###.",
        );

        let z = -2;
        let w = 1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = -1;
        let w = 1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 0;
        let w = 1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 1;
        let w = 1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 2;
        let w = 1;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = -2;
        let w = 2;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
..#..
.....
.....",
        );

        let z = -1;
        let w = 2;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 0;
        let w = 2;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
###..
##.##
#...#
.#..#
.###.",
        );

        let z = 1;
        let w = 2;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
.....
.....
.....",
        );

        let z = 2;
        let w = 2;
        dbg!(z, w);
        check_projection(
            &space,
            |hd| {
                (
                    Point::new(hd.x, hd.y),
                    hd.z == z + OFFSET && hd.w == w + OFFSET,
                )
            },
            "
.....
.....
..#..
.....
.....",
        );
    }
}

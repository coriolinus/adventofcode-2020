use aoc2020::geometry::{tile::DisplayWidth, vector3::Vector3, Map};

use std::{collections::HashSet, convert::TryFrom, path::Path};
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
pub struct ConwaySpace {
    // choose a sparse representation instead of extending map because this space is specifically infinite
    active: HashSet<Vector3>,
    min: Vector3,
    max: Vector3,
}

impl ConwaySpace {
    fn new<T>(input: T) -> Result<ConwaySpace, <Map<Cube> as TryFrom<T>>::Error>
    where
        Map<Cube>: TryFrom<T>,
    {
        let plane = Map::try_from(input)?;
        let expected_capacity = plane.width() * plane.height();
        let mut space = ConwaySpace::with_capacity(expected_capacity);
        plane.for_each_point(|&cube, point| {
            let point = Vector3::new(point.x, point.y, 0);
            if cube == Cube::Active {
                space.active.insert(point);
            }
            space.min = space.min.boundary_min(point);
            space.max = space.max.boundary_max(point);
        });
        Ok(space)
    }

    fn with_capacity(capacity: usize) -> ConwaySpace {
        ConwaySpace {
            active: HashSet::with_capacity(capacity),
            ..ConwaySpace::default()
        }
    }

    fn get(&self, point: Vector3) -> bool {
        self.active.contains(&point)
    }

    fn successor(&self) -> ConwaySpace {
        let mut successor =
            ConwaySpace::with_capacity((self.max - self.min).volume::<i64>() as usize);

        for point in Vector3::inclusive_range(self.min.decr(), self.max.incr()) {
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

    fn nth_successor(&self, n: usize) -> ConwaySpace {
        let mut successor = self.clone();

        for _ in 0..n {
            successor = successor.successor();
        }

        successor
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    const N: usize = 6;
    let mut space = ConwaySpace::new(input)?;
    space = space.nth_successor(N);
    let n_active = space.active.len();
    println!("{} active cubes after {} cycles", n_active, N);
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

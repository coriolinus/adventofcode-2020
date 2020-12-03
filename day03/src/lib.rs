use aoc2020::geometry::{Map, Point};

use std::convert::{TryFrom, TryInto};
use std::path::Path;
use thiserror::Error;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Tree,
    Clear,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Tree),
            '.' => Ok(Tile::Clear),
            other => Err(format!("invalid tile: {}", other)),
        }
    }
}

struct XWrapMap(Map<Tile>);

impl std::ops::Index<Point> for XWrapMap {
    type Output = Tile;

    fn index(&self, mut point: Point) -> &Tile {
        point.x %= self.0.width().try_into().unwrap_or(i32::MAX);
        self.0.index(point)
    }
}

fn count_trees(map: &XWrapMap, slope: Point) -> usize {
    let mut check = map.0.top_left();

    let mut n_trees = 0;
    while check.y >= 0 {
        if map[check] == Tile::Tree {
            n_trees += 1;
        }
        check += slope;
    }

    n_trees
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let map = XWrapMap(Map::try_from(input)?);
    let slope = Point::new(3, -1);
    let n_trees = count_trees(&map, slope);

    println!("trees encountered: {}", n_trees);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let map = XWrapMap(Map::try_from(input)?);

    let slopes = [
        Point::new(1, -1),
        Point::new(3, -1),
        Point::new(5, -1),
        Point::new(7, -1),
        Point::new(1, -2),
    ];

    let product: usize = slopes
        .iter()
        .map(|&slope| count_trees(&map, slope))
        .product();
    println!("product of trees encountered: {}", product);

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

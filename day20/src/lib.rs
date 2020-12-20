use aoc2020::{
    geometry::{tile::Bool, Direction, Map, Point},
    input::parse_newline_sep,
};

use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    path::Path,
    str::FromStr,
};
use thiserror::Error;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    parse_display::FromStr,
    parse_display::Display,
)]
#[display("Tile {0}:")]
struct TileId(u16);

#[derive(Clone, Default)]
struct Tile {
    id: u16,
    data: Map<Bool>,
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tile_id, tile_data) = match s.find('\n') {
            None => {
                return Err(Error::DetailParse(
                    s.to_string(),
                    "no newline in input".into(),
                ))
            }
            Some(idx) => s.split_at(idx),
        };

        Ok(Tile {
            id: tile_id.parse::<TileId>()?.0,
            data: (&tile_data[1..]).try_into()?,
        })
    }
}

#[inline]
fn reverse_edge(edge: u16, bit_width: usize) -> u16 {
    let mut output = 0;
    for bit_position in 0..bit_width {
        if edge & (1 << bit_position) != 0 {
            output |= 1 << (bit_width - bit_position - 1);
        }
    }
    output
}

/// A compact representation of a tile.
///
/// This makes it easier to fill the constraint problem, because it is small and easy
/// to manipulate.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
struct TileRepr {
    id: u16,
    /// These edges are arranged with the following convention:
    ///
    /// - Organization: `[Top, Right, Bottom, Left]`. This means that rotating the
    ///   tile can be accomplished by rotating the array.
    /// - Orientation: all edges are oriented clockwise. This makes it simple to rotate
    ///   the tile, but means that we need to reverse the right and bottom edge so that
    ///   they properly match the equivalent left and bottom edges when checking for
    ///   equality.
    edges: [u16; 4],
}

impl From<&Tile> for TileRepr {
    fn from(tile: &Tile) -> Self {
        let mut repr = TileRepr::default();

        repr.id = tile.id;

        // top
        repr.edges[0] =
            tile.data
                .edge(Direction::Up)
                .enumerate()
                .fold(0, |mut acc, (position, point)| {
                    if tile.data[point].into() {
                        acc |= 1 << position;
                    }
                    acc
                });

        // right
        repr.edges[1] = tile.data.edge(Direction::Right).rev().enumerate().fold(
            0,
            |mut acc, (position, point)| {
                if tile.data[point].into() {
                    acc |= 1 << position;
                }
                acc
            },
        );

        // bottom
        repr.edges[2] = tile.data.edge(Direction::Down).rev().enumerate().fold(
            0,
            |mut acc, (position, point)| {
                if tile.data[point].into() {
                    acc |= 1 << position;
                }
                acc
            },
        );

        // left
        repr.edges[3] =
            tile.data
                .edge(Direction::Left)
                .enumerate()
                .fold(0, |mut acc, (position, point)| {
                    if tile.data[point].into() {
                        acc |= 1 << position;
                    }
                    acc
                });

        repr
    }
}

impl TileRepr {
    fn side(self, direction: Direction, edge_width: usize) -> u16 {
        let (idx, reverse) = match direction {
            Direction::Up => (0, false),
            Direction::Right => (1, true),
            Direction::Down => (2, true),
            Direction::Left => (3, false),
        };

        let mut side = self.edges[idx];
        if reverse {
            side = reverse_edge(side, edge_width);
        }

        side
    }

    fn rotate_left(mut self, times: usize) -> Self {
        self.edges.rotate_left(times);
        self
    }

    fn rotate_right(mut self, times: usize) -> Self {
        self.edges.rotate_right(times);
        self
    }

    fn flip_horizontal(mut self, edge_width: usize) -> Self {
        // all edges are reversed when flipping: the ones oriented
        // along the flip direction because they are being explicitly
        // flipped, and the ones oriented against the flip direction
        // because opposite sides have opposite expected orientations.
        for edge in self.edges.iter_mut() {
            *edge = reverse_edge(*edge, edge_width);
        }
        self.edges.swap(1, 3);
        self
    }

    fn flip_vertical(mut self, edge_width: usize) -> Self {
        // all edges are reversed when flipping: the ones oriented
        // along the flip direction because they are being explicitly
        // flipped, and the ones oriented against the flip direction
        // because opposite sides have opposite expected orientations.
        for edge in self.edges.iter_mut() {
            *edge = reverse_edge(*edge, edge_width);
        }
        self.edges.swap(0, 2);
        self
    }

    fn all_orientations(self, edge_width: usize) -> impl Iterator<Item = TileRepr> {
        let flipped = self.flip_vertical(edge_width);
        (0..4)
            .map(move |n| self.rotate_left(n))
            .chain((0..4).map(move |n| flipped.rotate_left(n)))
    }
}

/// recursively try inserting tiles at the next available point in the map
fn insert_tile(
    map: &mut Map<Option<TileRepr>>,
    points: &[Point],
    available_tiles: &[TileRepr],
    used_tiles: &mut HashSet<u16>,
    edge_width: usize,
) -> bool {
    // if there is no more space to fill, then we must have succeeded
    if points.is_empty() {
        return true;
    }

    // otherwise, prepare for recursion
    let point = points[0];
    let points = &points[1..];

    'tile: for &tile in available_tiles {
        // can't re-use a tile
        if used_tiles.contains(&tile.id) {
            continue;
        }

        // it's a fresh tile, so plug it in and check validity
        map[point] = Some(tile);

        for direction in Direction::iter() {
            let adjacent = point + direction;
            // out of bounds tiles don't matter
            if !map.in_bounds(adjacent) {
                continue;
            }

            // if any adjacent tile is set but doesn't match this tile, then this tile is a dud;
            // continue with the next available tile
            if let Some(adjacent) = map[adjacent] {
                if tile.side(direction, edge_width)
                    != adjacent.side(direction.reverse(), edge_width)
                {
                    continue 'tile;
                }
            }
        }

        // at this point, there are no conflicts to putting this tile here. Recurse!
        used_tiles.insert(tile.id);
        if insert_tile(map, points, available_tiles, used_tiles, edge_width) {
            // we've found a complete solution! Don't mess with anything.
            return true;
        } else {
            // we ran into a dead end, so it's time to try the next tile. Before we do, clean up.
            used_tiles.remove(&tile.id);
        }
    }

    map[point] = None;
    false
}

fn arrange_tiles(tiles: impl IntoIterator<Item = Tile>) -> Result<Map<TileRepr>, Error> {
    let tiles: HashMap<_, _> = tiles.into_iter().map(|tile| (tile.id, tile)).collect();

    // compute edge width and validate that all tiles conform to it.
    let mut edge_width = None;
    for tile in tiles.values() {
        if tile.data.width() != tile.data.height() {
            return Err(Error::MalformedTile(tile.id));
        }
        match edge_width {
            None => edge_width = Some(tile.data.width()),
            Some(width) => {
                if tile.data.width() != width {
                    return Err(Error::MalformedTile(tile.id));
                }
            }
        }
    }

    let edge_width = match edge_width {
        Some(width) => width,
        None => return Ok(Map::default()),
    };

    let reprs: Vec<TileRepr> = tiles
        .values()
        .map(|tile| TileRepr::from(tile).all_orientations(edge_width))
        .flatten()
        .collect();

    let output_edge = (tiles.len() as f64).sqrt() as usize;
    let mut repr_map: Map<Option<TileRepr>> = Map::new(output_edge, output_edge);
    let mut used_tiles = HashSet::new();
    let points: Vec<_> = repr_map.points().collect();

    if insert_tile(&mut repr_map, &points, &reprs, &mut used_tiles, edge_width) {
        // convert repr_map into a new, better map
        let mut output_map: Map<TileRepr> = Map::new(output_edge, output_edge);
        for point in repr_map.points() {
            output_map[point] = repr_map[point].expect("all points in repr_map are filled");
        }
        Ok(output_map)
    } else {
        Err(Error::NoSolution)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let tiles_map = arrange_tiles(parse_newline_sep(input)?)?;
    let product: u64 = [
        tiles_map.top_left(),
        tiles_map.top_right(),
        tiles_map.bottom_left(),
        tiles_map.bottom_right(),
    ]
    .iter()
    .map(|point| tiles_map[*point].id as u64)
    .product();
    println!("product of ids of corners: {}", product);
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    Parse(#[from] parse_display::ParseError),
    #[error("err parsing {0:?}: {1}")]
    DetailParse(String, String),
    #[error("map converstion")]
    Map(#[from] aoc2020::geometry::map::MapConversionErr),
    #[error("not all tiles have equal width and height; {0} is bad")]
    MalformedTile(u16),
    #[error("no solution found")]
    NoSolution,
}

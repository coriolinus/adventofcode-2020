use aoc2020::parse;

use std::path::Path;
use thiserror::Error;

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[from_str(regex = "^(?P<0>[FB]{7}[LR]{3})$")]
struct BinarySpacePartition(String);

impl BinarySpacePartition {
    fn row(&self) -> u8 {
        let mut min = 0;
        let mut max = 127;
        for ch in self.0.chars().take(7) {
            let half_range = (max - min) / 2;
            match ch {
                'F' => max = min + half_range,
                'B' => min = max - half_range,
                _ => unreachable!("guaranteed by the parsing regex"),
            }
        }
        assert_eq!(min, max, "must have partitioned the range to a single row");
        min
    }

    fn col(&self) -> u8 {
        let mut min = 0;
        let mut max = 7;
        for ch in self.0.chars().skip(7).take(3) {
            let half_range = (max - min) / 2;
            match ch {
                'L' => max = min + half_range,
                'R' => min = max - half_range,
                _ => unreachable!("guaranteed by the parsing regex"),
            }
        }
        assert_eq!(min, max, "must have partitioned the range to a single row");
        min
    }

    fn seat_id(&self) -> u32 {
        (self.row() as u32 * 8) + self.col() as u32
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let highest = parse::<BinarySpacePartition>(input)?
        .map(|bsp| bsp.seat_id())
        .max();
    println!("highest seat id: {:?}", highest);
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

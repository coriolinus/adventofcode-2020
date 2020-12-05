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

fn find_empty_seat_id(map: &[bool]) -> Option<usize> {
    for (left_idx, window) in map.windows(3).enumerate() {
        if window[0] && !window[1] && window[2] {
            return Some(left_idx + 1);
        }
    }
    None
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut map = vec![false; 128 * 8];
    for boarding_pass in parse::<BinarySpacePartition>(input)? {
        map[boarding_pass.seat_id() as usize] = true;
    }

    let empty_seat_id = find_empty_seat_id(&map);
    println!("empty seat id: {:?}", empty_seat_id);

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

    fn check_seat(seat: &str, expect_row: u8, expect_col: u8, expect_id: u32) {
        let boarding_pass: BinarySpacePartition = seat.parse().unwrap();
        assert_eq!(boarding_pass.row(), expect_row);
        assert_eq!(boarding_pass.col(), expect_col);
        assert_eq!(boarding_pass.seat_id(), expect_id);
    }

    #[test]
    fn example_1() {
        check_seat("BFFFBBFRRR", 70, 7, 567);
    }

    #[test]
    fn example_2() {
        check_seat("FFFBBBFRRR", 14, 7, 119);
    }

    #[test]
    fn example_3() {
        check_seat("BBFFBBFRLL", 102, 4, 820);
    }
}

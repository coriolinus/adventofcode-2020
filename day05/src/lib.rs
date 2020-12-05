use aoc2020::parse;

use std::path::Path;
use thiserror::Error;

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[from_str(regex = "^(?P<0>[FB]{7}[LR]{3})$")]
struct BinarySpacePartition(String);

fn partition(chars: impl Iterator<Item = char>, n_chars: u32, lower: char, higher: char) -> u8 {
    let mut min = 0;
    let mut max = 2_u8.pow(n_chars) - 1;
    for ch in chars.take(n_chars as usize) {
        let half_range = (max - min) / 2;
        match ch {
            l if l == lower => max = min + half_range,
            h if h == higher => min = max - half_range,
            _ => unreachable!("guaranteed by the parsing regex"),
        }
    }
    assert_eq!(min, max, "must have partitioned the range to a single row");
    min
}

impl BinarySpacePartition {
    fn row(&self) -> u8 {
        partition(self.0.chars(), 7, 'F', 'B')
    }

    fn col(&self) -> u8 {
        partition(self.0.chars().skip(7), 3, 'L', 'R')
    }

    fn seat_id(&self) -> u32 {
        (self.row() as u32 * 8) + self.col() as u32
    }
}

fn find_empty_seat_id(map: &[bool]) -> Option<usize> {
    for (left_idx, window) in map.windows(3).enumerate() {
        if window[0] && !window[1] && window[2] {
            return Some(left_idx + 1);
        }
    }
    None
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let highest = parse::<BinarySpacePartition>(input)?
        .map(|bsp| bsp.seat_id())
        .max()
        .ok_or(Error::SolutionNotFound)?;
    println!("highest seat id: {}", highest);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut map = vec![false; 128 * 8];
    for boarding_pass in parse::<BinarySpacePartition>(input)? {
        map[boarding_pass.seat_id() as usize] = true;
    }

    let empty_seat_id = find_empty_seat_id(&map).ok_or(Error::SolutionNotFound)?;
    println!("empty seat id:   {}", empty_seat_id);

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("solution not found")]
    SolutionNotFound,
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

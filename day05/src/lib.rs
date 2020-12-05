use aoc2020::parse;

use std::path::Path;
use thiserror::Error;

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[from_str(regex = "^(?P<0>[FB]{7}[LR]{3})$")]
pub struct BoardingPass(String);

impl BoardingPass {
    pub fn row(&self) -> u16 {
        self.seat_id() >> 3
    }

    pub fn col(&self) -> u16 {
        self.seat_id() & 0b111
    }

    fn seat_id(&self) -> u16 {
        let mut out = 0;

        for (idx, ch) in self.0.chars().rev().enumerate() {
            if ch == 'B' || ch == 'R' {
                out |= 1 << idx;
            }
        }

        out
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
    let highest = parse::<BoardingPass>(input)?
        .map(|bsp| bsp.seat_id())
        .max()
        .ok_or(Error::SolutionNotFound)?;
    println!("highest seat id: {}", highest);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut map = vec![false; 128 * 8];
    for boarding_pass in parse::<BoardingPass>(input)? {
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

    fn check_seat(seat: &str, expect_row: u16, expect_col: u16, expect_id: u16) {
        let boarding_pass: BoardingPass = seat.parse().unwrap();
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

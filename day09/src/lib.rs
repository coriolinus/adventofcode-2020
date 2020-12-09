use aoc2020::parse;

use std::path::Path;
use thiserror::Error;

pub const DEFAULT_PREAMBLE_LEN: usize = 25;

/// True if the tail is the sum of any two of the previous `preamble_len` numbers
pub fn xmas_valid(items: &[u64], preamble_len: usize) -> bool {
    if items.len() < preamble_len {
        return true;
    }

    let preamble = &items[items.len().saturating_sub(preamble_len + 1)..items.len() - 1];
    let tail = items[items.len() - 1];

    for (idx, a) in preamble.iter().enumerate() {
        for b in &preamble[idx + 1..] {
            if a + b == tail {
                return true;
            }
        }
    }
    false
}

pub fn find_first_invalid(items: &[u64], preamble_len: usize) -> Option<u64> {
    for slice_end in preamble_len + 1..=items.len() {
        if !xmas_valid(&items[..slice_end], preamble_len) {
            return Some(items[slice_end - 1]);
        }
    }
    None
}

// never returns an empty slice
fn find_slice_with_sum<'a>(items: &'a [u64], target_sum: u64) -> Option<&'a [u64]> {
    for low in 0..items.len() {
        let mut running_sum = items[low];
        for high in low + 1..items.len() {
            running_sum += items[high];
            if running_sum == target_sum {
                return Some(&items[low..=high]);
            }
        }
    }
    None
}

pub fn find_weakness(items: &[u64], target_sum: u64) -> Option<u64> {
    let summing_slice = find_slice_with_sum(items, target_sum)?;
    // we know the slice is never empty, so we can just unwrap safely here
    Some(summing_slice.iter().min().unwrap() + summing_slice.iter().max().unwrap())
}

type Carryover = (Vec<u64>, u64);

fn compute_first_invalid(input: &Path) -> Result<Carryover, Error> {
    let items: Vec<u64> = parse(input)?.collect();
    let invalid = find_first_invalid(&items, DEFAULT_PREAMBLE_LEN).ok_or(Error::NoInvalid)?;
    Ok((items, invalid))
}

pub fn part1(input: &Path) -> Result<Carryover, Error> {
    let (items, invalid) = compute_first_invalid(input)?;
    println!("first invalid: {}", invalid);
    Ok((items, invalid))
}

pub fn part2(input: &Path, mut carryover: Option<Carryover>) -> Result<(), Error> {
    if carryover.is_none() {
        carryover = Some(compute_first_invalid(input)?);
    }
    let (items, invalid) = carryover.expect("we just guaranteed it wasn't none; qed");
    let weakness = find_weakness(&items, invalid).ok_or(Error::NoSliceWithSum)?;
    println!("encryption weakness: {}", weakness);

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no invalid item found")]
    NoInvalid,
    #[error("no sequence summing to target found")]
    NoSliceWithSum,
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        expect,
        case(26, true),
        case(49, true),
        case(100, false),
        case(50, false)
    )]
    fn test_validity_25(input: u64, expect: bool) {
        let mut items: Vec<u64> = (1..=25).collect();
        items.push(input);

        assert_eq!(xmas_valid(&items, 25), expect);
    }

    #[rstest(
        input,
        expect,
        case(26, true),
        case(65, false),
        case(64, true),
        case(66, true)
    )]
    fn test_26th(input: u64, expect: bool) {
        let mut items: Vec<u64> = (1..=25).collect();
        items.swap(0, 19);
        items.push(45);
        items.push(input);
        dbg!(&items);

        assert_eq!(xmas_valid(&items, 25), expect);
    }

    const SAMPLE_LIST: &[u64] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_find_first_invalid() {
        assert_eq!(find_first_invalid(&SAMPLE_LIST, 5), Some(127));
    }

    #[test]
    fn test_find_weakness() {
        assert_eq!(find_weakness(&SAMPLE_LIST, 127), Some(62));
    }
}

use aoc2020::parse;

use std::path::Path;
use thiserror::Error;

type Key = u32;

const MAX_ATTEMPTS: u32 = !0;
const MAGIC_DIVISOR: u64 = 20201227;
const PUBLIC_KEY_SUBJECT: Key = 7;

fn transform(loop_size: u32, subject_number: u32) -> u32 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number as u64;
        value %= MAGIC_DIVISOR;
    }
    value as u32
}

fn find_loop_size(subject_number: u32, key: Key) -> Option<u32> {
    let mut value = 1;
    for loop_size in 0..MAX_ATTEMPTS {
        if value == key as u64 {
            return Some(loop_size);
        }
        value *= subject_number as u64;
        value %= MAGIC_DIVISOR;
    }
    None
}

/// Compute the encryption key given the public keys
fn crack_given_keys((card, door): (Key, Key)) -> Result<Key, Error> {
    let card_loop_size =
        find_loop_size(PUBLIC_KEY_SUBJECT, card).ok_or(Error::NoSolution("card"))?;
    let encryption_key = transform(card_loop_size, door);

    // if in debug mode, double-check the invariants
    #[cfg(debug_assertions)]
    {
        let door_loop_size =
            find_loop_size(PUBLIC_KEY_SUBJECT, door).ok_or(Error::NoSolution("door"))?;
        let encryption_key_2 = transform(door_loop_size, card);
        assert_eq!(
            encryption_key, encryption_key_2,
            "card and door must compute same encryption key!"
        );
    }

    Ok(encryption_key)
}

fn parse_keys(input: &Path) -> Result<(Key, Key), Error> {
    let keys: Vec<Key> = parse(input)?.collect();
    if keys.len() != 2 {
        return Err(Error::MalformedInput);
    }
    Ok((keys[0], keys[1]))
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let encryption_key = crack_given_keys(parse_keys(input)?)?;
    println!("encryption key: {}", encryption_key);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("malformed input: must have two public keys, 1 per line")]
    MalformedInput,
    #[error("failed to crack {0} key to find its loop size")]
    NoSolution(&'static str),
}

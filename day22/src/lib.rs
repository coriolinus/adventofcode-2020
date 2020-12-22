use aoc2020::input::parse_newline_sep;

use regex::Regex;
use std::{path::Path, str::FromStr};
use thiserror::Error;

lazy_static::lazy_static! {
    static ref PLAYER_RE: Regex = Regex::new(r"^Player (\d+):$").unwrap();
}

struct Player {
    id: u8,
    cards: Vec<u8>,
}

impl FromStr for Player {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines
            .next()
            .ok_or_else(|| Error::ParsePlayer("no lines in player".into()))?;
        let id = &PLAYER_RE
            .captures(first_line)
            .ok_or_else(|| Error::ParsePlayer("player id regex didn't match".into()))?[1];
        let id: u8 = id
            .parse()
            .map_err(|err| Error::ParsePlayer(format!("parsing {:?} (id) as u8: {}", id, err)))?;

        let mut player = Player {
            id,
            cards: Vec::new(),
        };

        for line in lines.filter(|line| !line.is_empty()) {
            player.cards.push(line.parse().map_err(|err| {
                Error::ParsePlayer(format!("parsing {:?} (card) as u8: {}", id, err))
            })?);
        }

        Ok(player)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let players: Vec<Player> = parse_newline_sep(input)?.collect();
    println!("parsed {} players", players.len());
    unimplemented!()
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("malformed player: {0}")]
    ParsePlayer(String),
}

use aoc2020::input::parse_newline_sep;

use regex::Regex;
use std::{collections::VecDeque, path::Path, str::FromStr};
use thiserror::Error;

lazy_static::lazy_static! {
    static ref PLAYER_RE: Regex = Regex::new(r"^Player (\d+):$").unwrap();
}

struct Player {
    id: u8,
    cards: VecDeque<u8>,
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
            cards: VecDeque::new(),
        };

        for line in lines.filter(|line| !line.is_empty()) {
            player.cards.push_back(line.parse().map_err(|err| {
                Error::ParsePlayer(format!("parsing {:?} (card) as u8: {}", id, err))
            })?);
        }

        Ok(player)
    }
}

/// play a round of War ("Crab Combat"), returning the winner, if any
fn play_round(player1: &mut Player, player2: &mut Player) -> Option<u8> {
    let card1 = match player1.cards.pop_front() {
        Some(card) => (card, player1.id),
        None => return Some(player2.id),
    };
    let card2 = match player2.cards.pop_front() {
        Some(card) => (card, player2.id),
        None => return Some(player1.id),
    };

    let (winner_card, winner) = card1.max(card2);
    let (loser_card, _loser) = card1.min(card2);

    let (winner, loser) = if player1.id == winner {
        (player1, player2)
    } else {
        (player2, player1)
    };
    winner.cards.push_back(winner_card);
    winner.cards.push_back(loser_card);

    if loser.cards.is_empty() {
        Some(winner.id)
    } else {
        None
    }
}

fn play_until_victory(player1: &mut Player, player2: &mut Player) -> (usize, u8) {
    let mut rounds = 0;

    loop {
        rounds += 1;
        if let Some(winner) = play_round(player1, player2) {
            return (rounds, winner);
        }
    }
}

fn calculate_score_for(player1: &Player, player2: &Player, winner: u8) -> u32 {
    let winner = if player1.id == winner {
        player1
    } else {
        player2
    };

    winner
        .cards
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, &card)| {
            let score_multiplier = idx as u32 + 1;
            score_multiplier * card as u32
        })
        .sum()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut players: Vec<Player> = parse_newline_sep(input)?.collect();
    if players.len() != 2 {
        return Err(Error::WrongNumberPlayers(players.len()));
    }
    let mut player2 = players.swap_remove(1);
    let mut player1 = players.swap_remove(0);

    let (_n_rounds, winner) = play_until_victory(&mut player1, &mut player2);
    let score = calculate_score_for(&player1, &player2, winner);
    println!("victor score: {}", score);
    Ok(())
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
    #[error("wrong number of players: want 2, have {0}")]
    WrongNumberPlayers(usize),
}

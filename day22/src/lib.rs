use aoc2020::input::parse_newline_sep;

use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    path::Path,
    str::FromStr,
};
use thiserror::Error;

lazy_static::lazy_static! {
    static ref PLAYER_RE: Regex = Regex::new(r"^Player (\d+):$").unwrap();
}

#[derive(Clone)]
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

/// play a round of "Recursive Combat", returning the winner, if any
fn play_recursive(
    player1: &mut Player,
    player2: &mut Player,
    next_game: &mut usize,
    trace: bool,
) -> u8 {
    let game = *next_game;
    let mut memory = HashSet::new();
    if trace {
        println!("=== Game {} ===", game);
    }

    let mut round = 0;

    while !(player1.cards.is_empty() || player2.cards.is_empty()) {
        round += 1;
        if trace {
            println!("-- Round {} (Game {}) --", round, game);
            println!("player {}'s deck: {:?}", player1.id, player1.cards);
            println!("player {}'s deck: {:?}", player2.id, player2.cards);
        }

        if !memory.insert((player1.cards.clone(), player2.cards.clone())) {
            // insert returns `false` when the set already contains the item
            if trace {
                println!("game state already reached; player {} wins", player1.id);
            }
            return player1.id;
        }

        let card1 = match player1.cards.pop_front() {
            Some(card) => card,
            None => {
                if trace {
                    println!(
                        "Player {} out of cards; player {} wins",
                        player1.id, player2.id
                    );
                }
                return player2.id;
            }
        };
        let card2 = match player2.cards.pop_front() {
            Some(card) => card,
            None => {
                if trace {
                    println!(
                        "Player {} out of cards; player {} wins",
                        player2.id, player1.id
                    );
                }
                return player1.id;
            }
        };

        let winner;
        let winner_card;
        let loser_card;

        if trace {
            println!("Player 1 plays {}", card1);
            println!("Player 2 plays {}", card2);
        }

        if player1.cards.len() >= card1 as usize && player2.cards.len() >= card2 as usize {
            // play a complete sub-game to determine the winner of this round
            if trace {
                println!("Playing a sub-game to determine the winner...");
            }

            let mut sub_player1 = player1.clone();
            sub_player1.cards.truncate(card1 as usize);
            let mut sub_player2 = player2.clone();
            sub_player2.cards.truncate(card2 as usize);

            *next_game += 1;

            if player1.id == play_recursive(&mut sub_player1, &mut sub_player2, next_game, trace) {
                winner = &mut *player1;
                winner_card = card1;
                loser_card = card2;
            } else {
                winner = &mut *player2;
                winner_card = card2;
                loser_card = card1;
            }

            if trace {
                println!("...anyway, back to game {}", game);
            }
        } else {
            if card1 > card2 {
                winner = &mut *player1;
                winner_card = card1;
                loser_card = card2;
            } else {
                winner = &mut *player2;
                winner_card = card2;
                loser_card = card1;
            }
        }

        if trace {
            println!(
                "Player {} wins round {} of game {}!",
                winner.id, round, game
            );
        }

        winner.cards.push_back(winner_card);
        winner.cards.push_back(loser_card);
    }

    if player1.cards.is_empty() {
        if trace {
            println!("The winner of game {} is player {}", game, player2.id);
        }
        player2.id
    } else {
        if trace {
            println!("The winner of game {} is player {}", game, player1.id);
        }
        player1.id
    }
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

pub fn part2(input: &Path, trace: bool) -> Result<(), Error> {
    let mut players: Vec<Player> = parse_newline_sep(input)?.collect();
    if players.len() != 2 {
        return Err(Error::WrongNumberPlayers(players.len()));
    }
    let mut player2 = players.swap_remove(1);
    let mut player1 = players.swap_remove(0);
    let mut next_game = 1;

    let winner = play_recursive(&mut player1, &mut player2, &mut next_game, trace);
    let score = calculate_score_for(&player1, &player2, winner);
    println!("victor score (recursive): {}", score);
    Ok(())
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

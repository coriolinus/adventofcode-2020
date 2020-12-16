use std::{convert::TryFrom, path::Path};
use thiserror::Error;

mod model;
use model::{Input, Ticket, TicketField};

fn ticket_scanning_errors<'a>(
    fields: &'a [TicketField],
    check: &'a Ticket,
) -> impl 'a + Iterator<Item = u32> {
    check
        .iter()
        .filter(move |&value| !fields.iter().any(|field| field.contains(*value)))
        .copied()
}

fn ticket_scanning_error_rate(input: &Input) -> u32 {
    input
        .nearby_tickets
        .iter()
        .map(|ticket| ticket_scanning_errors(&input.fields, ticket))
        .flatten()
        .sum()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let input = Input::try_from(input)?;
    let error_rate = ticket_scanning_error_rate(&input);
    println!("ticket scanning error rate: {}", error_rate);
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parsing ticket field")]
    Field(#[from] parse_display::ParseError),
    #[error("parsing number")]
    Int(#[from] std::num::ParseIntError),
    #[error("input file had too many sections")]
    TooManySections,
    #[error("section \"{0}\" missing its initializer")]
    MissingInitial(usize),
}

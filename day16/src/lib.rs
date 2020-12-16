use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    path::Path,
};
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

fn valid_nearby_tickets(input: &Input) -> impl '_ + Iterator<Item = &Ticket> {
    input
        .nearby_tickets
        .iter()
        .filter(move |ticket| {
            ticket_scanning_errors(&input.fields, ticket)
                .next()
                .is_none()
        })
        .chain(std::iter::once(&input.my_ticket))
}

fn valid_indices_for_field<'a>(
    field: &'a TicketField,
    tickets: &'a [&Ticket],
    ticket_len: usize,
) -> impl 'a + Iterator<Item = usize> {
    (0..ticket_len).filter(move |&idx| tickets.iter().all(|ticket| field.contains(ticket[idx])))
}

fn analyze_tickets(input: &Input) -> HashMap<String, usize> {
    let valid_tickets: Vec<_> = valid_nearby_tickets(input).collect();
    let ticket_len = valid_tickets.get(0).map(|ticket| ticket.len());
    let mut mapping = HashMap::new();
    let mut known_indices = HashSet::new();

    if let Some(ticket_len) = ticket_len {
        let mut fields_to_check = input.fields.clone();
        let mut potential_indices = Vec::with_capacity(ticket_len);

        while !fields_to_check.is_empty() {
            fields_to_check.retain(|field| {
                potential_indices.clear();
                potential_indices.extend(
                    valid_indices_for_field(field, &valid_tickets, ticket_len)
                        .filter(|idx| !known_indices.contains(idx)),
                );
                match potential_indices.len() {
                    0 => panic!("no more potential indices for field {}", field.name),
                    1 => {
                        mapping.insert(field.name.clone(), potential_indices[0]);
                        known_indices.insert(potential_indices[0]);
                        false
                    }
                    _ => true,
                }
            })
        }
    }

    mapping
}

fn departure_product(input: &Input) -> u64 {
    let mapping = analyze_tickets(input);
    mapping
        .iter()
        .filter(|(key, _)| key.starts_with("departure"))
        .map(|(_, &idx)| input.my_ticket[idx] as u64)
        .product()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let input = Input::try_from(input)?;
    let error_rate = ticket_scanning_error_rate(&input);
    println!("ticket scanning error rate: {}", error_rate);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let input = Input::try_from(input)?;
    let departure_product = departure_product(&input);
    println!("departure product: {}", departure_product);
    Ok(())
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

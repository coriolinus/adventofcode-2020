use lalrpop_util::lalrpop_mod;
use std::{collections::HashMap, path::Path};
use thiserror::Error;

lalrpop_mod!(parser);

type Ident = usize;

pub enum RuleTerm {
    Literal(char),
    Subrules(Vec<Ident>),
}

impl RuleTerm {
    fn as_mut_subrules(&mut self) -> Option<&mut Vec<Ident>> {
        match self {
            Self::Literal(_) => None,
            Self::Subrules(rules) => Some(rules),
        }
    }
}

pub struct Rule {
    ident: Ident,
    alternates: Vec<RuleTerm>,
}

type Message = String;

pub struct Input {
    rules: HashMap<Ident, Rule>,
    messages: Vec<Message>,
}

fn parse_path(input: &Path) -> Result<Input, Error> {
    let data = std::fs::read_to_string(input)?;
    parser::InputParser::new()
        .parse(&data)
        .map_err(|err| Error::Parse(Box::new(err.map_token(|t| t.to_string()))))
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let input = parse_path(input)?;
    println!(
        "parsed input: {} rules; {} messages",
        input.rules.len(),
        input.messages.len()
    );
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    Parse(#[source] Box<dyn std::error::Error + Send + Sync>),
}

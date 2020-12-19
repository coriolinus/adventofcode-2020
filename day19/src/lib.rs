use std::{convert::TryFrom, path::Path};
use thiserror::Error;

mod ast;
use ast::Input;

pub fn part1(input: &Path) -> Result<(), Error> {
    let input = Input::try_from(input)?;
    println!(
        "parse success: {} rules; {} messages",
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
    #[error(transparent)]
    Num(#[from] std::num::ParseIntError),
    #[error("parse error: in \"{0}\", {1}")]
    Parse(String, String),
}

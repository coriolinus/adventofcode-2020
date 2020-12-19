use crate::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

lazy_static! {
    static ref TERM_LITERAL: Regex = Regex::new(r#""(\w)""#).unwrap();
}

type Ident = usize;

type Subrules = Vec<Ident>;

enum RuleTerm {
    Literal(char),
    Subrules(Vec<Subrules>),
}

impl FromStr for RuleTerm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

struct Rule {
    ident: Ident,
    term: RuleTerm,
}

type Message = String;

pub struct Input {
    rules: HashMap<Ident, Rule>,
    messages: Vec<Message>,
}

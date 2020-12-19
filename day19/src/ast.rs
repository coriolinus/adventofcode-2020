use crate::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, convert::TryFrom, path::Path, str::FromStr};

lazy_static! {
    static ref TERM_LITERAL: Regex = Regex::new(r#""(\w)""#).unwrap();
    static ref RULE: Regex = Regex::new(r"^(\d+): (.*)$").unwrap();
}

pub type Ident = usize;

pub type Subrules = Vec<Ident>;

pub enum RuleTerm {
    Literal(char),
    Subrules(Vec<Subrules>),
}

impl FromStr for RuleTerm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(capture) = TERM_LITERAL.captures(s) {
            Ok(RuleTerm::Literal(
                capture[1]
                    .chars()
                    .next()
                    .expect("regex guarantees at least 1 char; qed"),
            ))
        } else {
            let mut subrules = Vec::new();
            let mut current_subrule = Vec::new();

            for token in s.split_whitespace() {
                if token == "|" {
                    subrules.push(std::mem::take(&mut current_subrule));
                } else {
                    current_subrule.push(token.parse()?);
                }
            }

            subrules.push(current_subrule);

            Ok(RuleTerm::Subrules(subrules))
        }
    }
}

pub struct Rule {
    pub ident: Ident,
    pub term: RuleTerm,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RULE
            .captures(s)
            .ok_or_else(|| Error::Parse(s.to_string(), "did not match RULE regex".to_string()))?;
        let ident = captures[1].parse()?;
        let term = captures[2].parse()?;
        Ok(Rule { ident, term })
    }
}

type Message = String;

#[derive(Default)]
pub struct Input {
    pub rules: HashMap<Ident, Rule>,
    pub messages: Vec<Message>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = Input::default();

        for (idx, section) in s.split("\n\n").enumerate() {
            match idx {
                0 => {
                    // rules
                    for rule in section.split('\n') {
                        let rule: Rule = rule.parse()?;
                        input.rules.insert(rule.ident, rule);
                    }
                }
                1 => {
                    // messages
                    input.messages = section.split('\n').map(|msg| msg.to_string()).collect();
                }
                _ => {
                    return Err(Error::Parse(
                        section[..50].to_string(),
                        "more sections than expected".to_string(),
                    ));
                }
            }
        }

        Ok(input)
    }
}

impl TryFrom<&Path> for Input {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let data = std::fs::read_to_string(value)?;
        data.parse()
    }
}

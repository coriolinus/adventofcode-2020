use std::{collections::HashMap, convert::TryFrom, path::Path};
use thiserror::Error;

mod ast;
use ast::{Ident, Input, Rule, RuleTerm};

/// returns a list of portions remaining after successful matches of alternatives.
///
/// If no alternative matches, returns an empty list
///
/// Success can be defined as "at least one empty string is among the output"
fn matches_rule<'a>(rule: Ident, rules: &HashMap<Ident, Rule>, input: &'a str) -> Vec<&'a str> {
    if input.is_empty() {
        return Vec::new();
    }
    let rule = match rules.get(&rule) {
        Some(rule) => rule,
        None => return Vec::new(),
    };

    match &rule.term {
        RuleTerm::Literal(ch) => {
            if input.chars().next() == Some(*ch) {
                return vec![&input[ch.len_utf8()..]];
            }
        }
        RuleTerm::Subrules(subrules) => {
            let mut matching_remaining_input = Vec::new();
            for subrule in subrules.iter() {
                let mut remaining_input = vec![input];
                for term in subrule.iter() {
                    let mut new_remaining_input = Vec::new();
                    for input in remaining_input {
                        new_remaining_input.extend(matches_rule(*term, rules, input));
                    }
                    remaining_input = new_remaining_input;
                }
                matching_remaining_input.extend(remaining_input);
            }
            matching_remaining_input.sort();
            matching_remaining_input.dedup();
            return matching_remaining_input;
        }
    }
    Vec::new()
}

fn matches_rule_0(input: &Input) -> impl '_ + Iterator<Item = &String> {
    input.messages.iter().filter(move |msg| {
        matches_rule(0, &input.rules, msg)
            .iter()
            .any(|remainder| remainder.is_empty())
    })
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let input = Input::try_from(input)?;
    let n_matches_0 = matches_rule_0(&input).count();
    println!("number matches rule 0: {}", n_matches_0);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut input = Input::try_from(input)?;
    input.rules.insert(
        8,
        Rule {
            ident: 8,
            term: RuleTerm::Subrules(vec![vec![42], vec![42, 8]]),
        },
    );
    input.rules.insert(
        11,
        Rule {
            ident: 11,
            term: RuleTerm::Subrules(vec![vec![42, 31], vec![42, 11, 31]]),
        },
    );
    let n_matches_0 = matches_rule_0(&input).count();
    println!("number matches rule 0 (modified rules): {}", n_matches_0);
    Ok(())
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

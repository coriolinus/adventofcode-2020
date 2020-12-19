use std::{collections::HashMap, convert::TryFrom, path::Path};
use thiserror::Error;

mod ast;
use ast::{Ident, Input, Rule, RuleTerm};

/// returns whether this rule matched, and the unconsumed portion of the `input` str
///
/// When there is no match, the output str is always equal to the input str.
fn matches_rule<'a>(rule: Ident, rules: &HashMap<Ident, Rule>, input: &'a str) -> (bool, &'a str) {
    if input.is_empty() {
        return (false, input);
    }
    let rule = match rules.get(&rule) {
        Some(rule) => rule,
        None => return (false, input),
    };

    match &rule.term {
        RuleTerm::Literal(ch) => {
            if input.chars().next() == Some(*ch) {
                return (true, &input[ch.len_utf8()..]);
            }
        }
        RuleTerm::Subrules(subrules) => {
            let mut non_empty_matching_remaining_input = Vec::new();
            'outer: for subrule in subrules.iter() {
                let mut remaining_input = input;
                for term in subrule.iter() {
                    let (matches, remaining) = matches_rule(*term, rules, remaining_input);
                    if !matches {
                        continue 'outer;
                    }
                    remaining_input = remaining;
                }
                // if we haven't continued past this point, then all terms in this subrule matched.
                if remaining_input.is_empty() {
                    return (true, remaining_input);
                }
                non_empty_matching_remaining_input.push(remaining_input);
            }
            if non_empty_matching_remaining_input.len() > 1 {
                println!("WARN: non_empty_matching_remaining_input has len {}, but we arbitrarily return only the first", non_empty_matching_remaining_input.len());
            }
            if !non_empty_matching_remaining_input.is_empty() {
                return (true, non_empty_matching_remaining_input[0]);
            }
        }
    }
    (false, input)
}

fn matches_rule_0(input: &Input) -> impl '_ + Iterator<Item = &String> {
    input.messages.iter().filter(move |msg| {
        let (matches, remainder) = matches_rule(0, &input.rules, msg);
        matches && remainder.is_empty()
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

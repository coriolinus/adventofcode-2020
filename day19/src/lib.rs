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
                // it _might_ be safe to just return early in this case--but there might be a need for
                // recursive backtracking. In case part 1 has the wrong answer, look here first!
                //
                // ... it was the right answer. Guess it worked?
                return (true, remaining_input);
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

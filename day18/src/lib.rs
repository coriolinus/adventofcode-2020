use aoc2020::parse;
use lalrpop_util::lalrpop_mod;

use std::{
    ops::{Add, Mul},
    path::Path,
    str::FromStr,
};
use thiserror::Error;

lalrpop_mod!(parser);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn apply_to<T>(self, a: T, b: T) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        match self {
            Operation::Add => a + b,
            Operation::Mul => a * b,
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Operation::Add
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Literal(i64),
    Expression(Box<Expr>),
}

impl Value {
    pub fn value(&self) -> i64 {
        match self {
            Value::Literal(n) => *n,
            Value::Expression(e) => e.value(),
        }
    }

    // reduce this value to a literal using advanced math
    fn reduce_advanced(&self) -> i64 {
        match self {
            Value::Literal(n) => *n,
            Value::Expression(expr) => expr.evaluate_advanced(),
        }
    }
}

impl<T> From<T> for Value
where
    i64: From<T>,
{
    fn from(t: T) -> Self {
        Value::Literal(t.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    operation: Operation,
    value: Value,
}

impl Term {
    fn apply_to(&self, value: i64) -> i64 {
        self.operation.apply_to(self.value.value(), value)
    }
}

impl From<Value> for Term {
    fn from(value: Value) -> Self {
        Term {
            operation: Operation::default(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    terms: Vec<Term>,
}

impl FromStr for Expr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::ExprParser::new()
            .parse(s)
            .map_err(|e| Error::Parse(Box::new(e.map_token(|t| t.to_string()))))
    }
}

impl Expr {
    fn evaluate(terms: &[Term]) -> i64 {
        terms.iter().fold(0, |acc, elem| elem.apply_to(acc))
    }

    pub fn value(&self) -> i64 {
        Self::evaluate(&self.terms)
    }

    /// In "advanced" math, addition is evaluated _before_ multiplication.
    pub fn evaluate_advanced(&self) -> i64 {
        // convert to a list of multiplications
        let remaining_terms = self.terms.iter().fold(Vec::new(), |mut remaining, term| {
            // first term
            if remaining.is_empty() {
                remaining.push(term.value.reduce_advanced());
                return remaining;
            }

            match term.operation {
                Operation::Add => {
                    // fold this term's value into the final term of remaining
                    let last = remaining.len() - 1;
                    remaining[last] += term.value.reduce_advanced();
                }
                Operation::Mul => {
                    remaining.push(term.value.reduce_advanced());
                }
            }
            remaining
        });
        // evaluate that list
        remaining_terms.iter().product()
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let sum = parse::<Expr>(input)?.map(|expr| expr.value()).sum::<i64>();
    println!("sum of expressions: {}", sum);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let sum = parse::<Expr>(input)?
        .map(|expr| expr.evaluate_advanced())
        .sum::<i64>();
    println!("sum of expressions (advanced): {}", sum);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    Parse(#[source] Box<dyn std::error::Error + Send + Sync>),
}

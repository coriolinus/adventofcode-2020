use aoc2020::parse;

use std::{
    ops::{Add, Mul},
    path::Path,
};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
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
    Literal(i32),
    Expression(Box<Expr>),
}

impl Value {
    pub fn value(&self) -> i32 {
        match self {
            Value::Literal(n) => *n,
            Value::Expression(e) => e.value(),
        }
    }
}

impl<T> From<T> for Value
where
    i32: From<T>,
{
    fn from(t: T) -> Self {
        Value::Literal(t.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Term {
    operation: Operation,
    value: Value,
}

impl Term {
    fn apply_to(&self, value: i32) -> i32 {
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

impl Expr {
    pub fn value(&self) -> i32 {
        self.terms.iter().fold(0, |acc, elem| elem.apply_to(acc))
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    unimplemented!()
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

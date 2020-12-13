//! The **Chinese Remainder Theorem** is used to efficiently calculate a number `n` which
//! has several constraints of its modulus. Each constraint specifies the modulus and the
//! remainder, such that:
//!
//! ```text
//! n % constraints[0].modulus == constraints[0].remainder
//! n % constraints[1].modulus == constraints[1].remainder
//! ...
//! n % constraints[k].modulus == constraints[k].remainder
//! ```
//!
//! This module is adapted from the example in [Rosetta Code](https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust)

use num::{integer::Integer, traits::Signed};
use std::{iter::Product, ops::AddAssign};

fn egcd<N: Integer + Copy + Signed>(a: N, b: N) -> (N, N, N) {
    if a.is_zero() {
        (b, N::zero(), N::one())
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv<N: Integer + Copy + Signed>(x: N, n: N) -> Option<N> {
    let (g, x, _) = egcd(x, n);
    if g.is_one() {
        Some((x % n + n) % n)
    } else {
        None
    }
}

/// A constraint for the calculation of the Chinese Remainder Theorem
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Constraint<N> {
    pub modulus: N,
    pub remainder: N,
}

impl<N> From<(N, N)> for Constraint<N> {
    /// Assumes that constraints are ordered `(modulus, remainder)`.
    fn from((modulus, remainder): (N, N)) -> Constraint<N> {
        Constraint { modulus, remainder }
    }
}

impl<N> Constraint<N> {
    pub fn new(modulus: N, remainder: N) -> Constraint<N> {
        Constraint { modulus, remainder }
    }
}

impl<N> Constraint<N>
where
    N: Copy + Integer,
{
    /// This formulation is useful when what's available is the "inverted remainder":
    /// `invert_remainder == modulus - remainder`.
    pub fn new_invert_remainder(modulus: N, invert_remainder: N) -> Constraint<N> {
        Constraint::new(modulus, (modulus - invert_remainder) % modulus)
    }
}

/// Find a number `n` which follows the supplied constraints.
///
/// These constraints are expressed such that for all `k` in `(0..constraints.len())`:
///
/// ```text
/// n % constraints[0].modulus == constraints[0].remainder
/// n % constraints[1].modulus == constraints[1].remainder
/// ...
/// n % constraints[k].modulus == constraints[k].remainder
/// ```
///
/// Returns `None` if the constraint moduli are not all coprime.
pub fn chinese_remainder<N>(constraints: &[Constraint<N>]) -> Option<N>
where
    N: Integer + Copy + Product + AddAssign + Signed,
{
    let product = constraints
        .iter()
        .map(|&constraint| constraint.modulus)
        .product::<N>();

    let mut sum = N::zero();

    for Constraint { modulus, remainder } in constraints {
        let p = product / *modulus;
        sum += *remainder * mod_inv(p, *modulus)? * p;
    }

    Some(sum % product)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_classic_formulation() {
        let constraints = [
            Constraint {
                modulus: 3,
                remainder: 2,
            },
            Constraint {
                modulus: 5,
                remainder: 3,
            },
            Constraint {
                modulus: 7,
                remainder: 2,
            },
        ];

        let n = chinese_remainder(&constraints).unwrap();
        assert_eq!(n, 23);
    }

    #[test]
    fn test_worked_inverted() {
        let constraints = [
            Constraint::new_invert_remainder(7, 0),
            Constraint::new_invert_remainder(13, 1),
            Constraint::new_invert_remainder(59, 4),
            Constraint::new_invert_remainder(31, 6),
            Constraint::new_invert_remainder(19, 7),
        ];

        dbg!(&constraints);

        let expect = 1068781;
        let n = chinese_remainder(&constraints).unwrap();
        dbg!(n);
        for constraint in &constraints {
            dbg!(
                constraint.modulus,
                n % constraint.modulus,
                expect % constraint.modulus
            );
        }
        assert_eq!(n, expect);
    }

    #[test]
    fn test_worked_literals() {
        let constraints = [
            Constraint {
                modulus: 7,
                remainder: 0,
            },
            Constraint {
                modulus: 13,
                remainder: 12,
            },
            Constraint {
                modulus: 59,
                remainder: 55,
            },
            Constraint {
                modulus: 31,
                remainder: 25,
            },
            Constraint {
                modulus: 19,
                remainder: 12,
            },
        ];

        let expect = 1068781;
        let n = chinese_remainder(&constraints).unwrap();
        dbg!(n);
        for constraint in &constraints {
            dbg!(
                constraint.modulus,
                n % constraint.modulus,
                expect % constraint.modulus
            );
        }
        assert_eq!(n, expect);
    }

    #[test]
    fn test_another() {
        let constraints = [
            Constraint::new_invert_remainder(17, 0),
            Constraint::new_invert_remainder(13, 2),
            Constraint::new_invert_remainder(19, 3),
        ];

        let n = chinese_remainder(&constraints).unwrap();
        for constraint in &constraints {
            dbg!(constraint.modulus, n % constraint.modulus);
        }
        assert_eq!(n, 3417);
    }
}

use aoc2020::parse;

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    str::FromStr,
};
use thiserror::Error;

lazy_static! {
    static ref FOOD_RE: Regex = Regex::new(r"([\w\s]*)\(contains ([\w\s,]+)\)").unwrap();
}

struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Food {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = FOOD_RE.captures(s).ok_or(Error::ParseError)?;
        let ingredients = captures[1]
            .split_whitespace()
            .map(|ingredient| ingredient.to_string())
            .collect();
        let allergens = captures[2]
            .split(',')
            .map(|allergen| allergen.trim().to_string())
            .collect();
        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

/// we can create a map of plausible allergens by this process:
///
/// - create the combined set of all known allergens
/// - for each known allergen:
///   - note all ingredients which appear every time the allergen appears
fn plausible_allergens(foods: &[Food]) -> HashMap<String, HashSet<String>> {
    let allergens =
        foods
            .iter()
            .map(|food| &food.allergens)
            .fold(HashSet::new(), |mut acc, elem| {
                acc.extend(elem.iter().cloned());
                acc
            });

    let mut allergen_map = HashMap::new();

    for allergen in allergens.iter() {
        let plausible_ingredients = foods
            .iter()
            .filter(|food| food.allergens.contains(allergen))
            .fold(None, |acc, food| match acc {
                None => Some(food.ingredients.clone()),
                Some(plausible_ingredients) => Some(
                    plausible_ingredients
                        .intersection(&food.ingredients)
                        .cloned()
                        .collect(),
                ),
            });

        if let Some(ingredients) = plausible_ingredients {
            if !ingredients.is_empty() {
                allergen_map.insert(allergen.clone(), ingredients);
            }
        }
    }

    allergen_map
}

fn implausible_allergens<'a>(
    foods: &'a [Food],
    plausible: &'a HashMap<String, HashSet<String>>,
) -> impl 'a + Iterator<Item = String> {
    foods
        .iter()
        .map(move |food| {
            food.ingredients
                .iter()
                .filter(move |&ingredient| !plausible.contains_key(ingredient))
        })
        .flatten()
        .cloned()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let foods: Vec<Food> = parse(input)?.collect();
    let plausible = plausible_allergens(&foods);
    let n_implausible = implausible_allergens(&foods, &plausible).count();
    println!("{} foods implausible as allergens", n_implausible);
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("input did not match parsing regex")]
    ParseError,
}

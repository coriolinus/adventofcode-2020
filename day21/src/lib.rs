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
///
/// Creates a map of `allergen => [ingredient]`
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
            food.ingredients.iter().filter(move |&ingredient| {
                !plausible.values().any(|values| values.contains(ingredient))
            })
        })
        .flatten()
        .cloned()
}

// `ingredient => allergen`
fn identify_allergens(foods: &[Food]) -> HashMap<String, String> {
    let mut plausible = plausible_allergens(foods);

    // remove inert ingredients from the list of plausible ingredients for each allergen
    let inerts: HashSet<_> = implausible_allergens(foods, &plausible).collect();
    for plausible_set in plausible.values_mut() {
        *plausible_set = plausible_set.difference(&inerts).cloned().collect();
    }

    let mut allergens = HashMap::new();
    let mut newly_known_ingredients = HashSet::new();

    while !plausible.is_empty() {
        for (allergen, possible_ingredients) in plausible.iter_mut() {
            if possible_ingredients.len() == 1 {
                let ingredient = possible_ingredients.drain().next().unwrap();
                newly_known_ingredients.insert(ingredient.clone());
                allergens.insert(ingredient, allergen.clone());
            }
        }
        for plausible_set in plausible.values_mut() {
            *plausible_set = plausible_set
                .difference(&newly_known_ingredients)
                .cloned()
                .collect();
        }

        debug_assert!(newly_known_ingredients.len() > 0);
        newly_known_ingredients.clear();

        plausible.retain(|_allergen, possible_ingredients| possible_ingredients.len() > 0);
    }

    allergens
}

fn canonical_dangerous_ingredient_list(foods: &[Food]) -> String {
    let allergens = identify_allergens(foods);
    let mut allergens: Vec<_> = allergens
        .into_iter()
        .map(|(ingredient, allergen)| (allergen, ingredient))
        .collect();
    allergens.sort();
    let allergens: Vec<_> = allergens
        .into_iter()
        .map(|(_allergen, ingredient)| ingredient)
        .collect();
    allergens.join(",")
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let foods: Vec<Food> = parse(input)?.collect();
    let plausible = plausible_allergens(&foods);
    let n_implausible = implausible_allergens(&foods, &plausible).count();
    println!("{} foods implausible as allergens", n_implausible);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let foods: Vec<Food> = parse(input)?.collect();
    let canonical_dangerous_ingredients = canonical_dangerous_ingredient_list(&foods);
    println!(
        "canonical dangerous ingredient list: {}",
        canonical_dangerous_ingredients
    );
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("input did not match parsing regex")]
    ParseError,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
    mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)
    ";

    fn example() -> Vec<Food> {
        EXAMPLE
            .trim()
            .lines()
            .map(|line| line.parse().expect("food is properly defined"))
            .collect()
    }

    #[test]
    fn test_example() {
        let foods = example();
        let plausible = dbg!(plausible_allergens(&foods));
        let mut implausible_ingredients: Vec<_> =
            implausible_allergens(&foods, &plausible).collect();

        dbg!(&implausible_ingredients);

        assert_eq!(implausible_ingredients.len(), 5);
        implausible_ingredients.sort();
        implausible_ingredients.dedup();

        assert_eq!(
            implausible_ingredients,
            vec![
                "kfcds".to_string(),
                "nhms".to_string(),
                "sbzzf".to_string(),
                "trh".to_string(),
            ],
        );
    }
}

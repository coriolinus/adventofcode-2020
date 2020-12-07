use aoc2020::parse;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use thiserror::Error;

lazy_static! {
    static ref LUGGAGE_OUTER_RE: Regex = Regex::new(r"^(?P<outer_color>.*) bags contain").unwrap();
    static ref LUGGAGE_INNER_RE: Regex =
        Regex::new(r"(?P<qty>\d+) (?P<color>[^,.]*) bags?[.,]").unwrap();
}

const MY_BAG: &str = "shiny gold";

pub struct LuggageRule {
    outer_color: String,
    contents: Vec<(u32, String)>,
}

impl std::str::FromStr for LuggageRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outer_color = LUGGAGE_OUTER_RE
            .captures(s)
            .ok_or_else(|| "could not find outer color".to_string())?["outer_color"]
            .to_string();

        let contents = LUGGAGE_INNER_RE
            .captures_iter(s)
            .map(|capture| {
                (
                    capture["qty"]
                        .parse::<u32>()
                        .expect("regex guarantees positive integers"),
                    capture["color"].to_string(),
                )
            })
            .collect();

        Ok(LuggageRule {
            outer_color,
            contents,
        })
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut direct_containers: HashMap<String, HashSet<String>> = HashMap::new();
    for rule in parse::<LuggageRule>(input)? {
        for (_, contained_bag) in &rule.contents {
            direct_containers
                .entry(contained_bag.clone())
                .or_default()
                .insert(rule.outer_color.clone());
        }
    }

    let mut queue: VecDeque<_> = direct_containers[MY_BAG].iter().cloned().collect();
    let mut all_containers = HashSet::new();
    while let Some(q) = queue.pop_front() {
        if all_containers.insert(q.clone()) {
            // true if the value was not present in the set
            queue.extend(direct_containers.entry(q).or_default().iter().cloned());
        }
    }

    println!(
        "{} bags can eventually contain a {} bag",
        all_containers.len(),
        MY_BAG
    );

    Ok(())
}

fn query_rules(rules: &HashMap<String, LuggageRule>, color: &str) -> u64 {
    let rule = match rules.get(color) {
        None => return 0,
        Some(rule) => rule,
    };

    let mut qty_contained = 0_u64;

    for (qty, color) in &rule.contents {
        let qty = *qty as u64;
        qty_contained += qty;
        qty_contained += qty * query_rules(rules, color);
    }

    qty_contained
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let rules: HashMap<_, _> = parse::<LuggageRule>(input)?
        .map(|rule| (rule.outer_color.clone(), rule))
        .collect();
    let total_contained = query_rules(&rules, MY_BAG);
    println!("my bag contains {} other bags", total_contained);

    Ok(())
}

fn query_rules_memoize(
    rules: &HashMap<String, LuggageRule>,
    memo: &mut HashMap<String, u64>,
    color: &str,
) -> u64 {
    let rule = match rules.get(color) {
        None => return 0,
        Some(rule) => rule,
    };

    let mut qty_contained = 0;

    for (qty, color) in &rule.contents {
        let qty = *qty as u64;
        qty_contained += qty;
        let per_color = match memo.get(color) {
            Some(n) => *n,
            None => query_rules_memoize(rules, memo, color),
        };
        qty_contained += qty * per_color;
    }

    memo.insert(color.to_string(), qty_contained);
    qty_contained
}

pub fn exhaustive_quantize(input: &Path, n: usize) -> Result<(), Error> {
    let rules: HashMap<_, _> = parse::<LuggageRule>(input)?
        .map(|rule| (rule.outer_color.clone(), rule))
        .collect();

    let mut exhaustive_contents = HashMap::new();
    for color in rules.keys() {
        query_rules_memoize(&rules, &mut exhaustive_contents, color);
    }

    let mut exhaustive: Vec<_> = exhaustive_contents.iter().map(|(k, v)| (v, k)).collect();
    exhaustive.sort();

    for (n, color) in exhaustive.iter().rev().take(n) {
        println!("{:>30}: {:6}", color, n);
    }
    println!("{:>31}", "...");
    println!("{:>30}: {:6}", MY_BAG, exhaustive_contents[MY_BAG]);
    println!("{:>31}", "...");
    for (n, color) in exhaustive.iter().take(n).rev() {
        println!("{:>30}: {:6}", color, n);
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

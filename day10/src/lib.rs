use aoc2020::parse;

use counter::Counter;
use std::path::Path;
use thiserror::Error;

const CHARGING_OUTLET: u32 = 0;

fn make_adapter_chain(adapters: &[u32]) -> Option<Vec<u32>> {
    let adapters_len = adapters.len();
    let mut adapters = adapters.to_vec();

    // insert charging outlet
    adapters.push(CHARGING_OUTLET);
    adapters.sort();

    // insert device internal adapter
    adapters.push(adapters[adapters.len() - 1] + 3);

    // truncate if the chain gets out of range
    if let Some((left_idx, _)) = adapters
        .windows(2)
        .enumerate()
        .find(|(_, window)| window[1] > window[0] + 3)
    {
        adapters.truncate(left_idx + 1);
    }

    if adapters_len + 2 == adapters.len() {
        Some(adapters)
    } else {
        None
    }
}

fn adapter_chain_stats(adapters: &[u32]) -> Option<Counter<u32>> {
    make_adapter_chain(adapters).map(|adapters| {
        adapters
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect()
    })
}

fn count_legal_adapter_arrangements(adapters: &[u32]) -> usize {
    if adapters.is_empty() {
        return 0;
    }

    // we know the adapter chain isn't empty, so it's safe to unwrap here
    let unused_adapters = make_adapter_chain(adapters).unwrap();

    let mut memoize = Vec::with_capacity(unused_adapters.len());
    let mut adapters = vec![unused_adapters[0]];
    let unused_adapters = &unused_adapters[1..];

    count_legal_adapter_arrangements_memoized(&mut adapters, unused_adapters, &mut memoize)
}

fn count_legal_adapter_arrangements_memoized(
    adapters: &mut Vec<u32>,
    unused_adapters: &[u32],
    memoize: &mut Vec<usize>,
) -> usize {
    if let Some(memo) = memoize.get(unused_adapters.len()) {
        return *memo;
    }

    let n_legal_successors =
        count_legal_adapter_arrangements_inner(adapters, unused_adapters, memoize);

    if memoize.len() == unused_adapters.len() {
        memoize.push(n_legal_successors);
    }

    n_legal_successors
}

// precondition: never call with empty `adapters`
fn count_legal_adapter_arrangements_inner(
    adapters: &mut Vec<u32>,
    mut unused_adapters: &[u32],
    memoize: &mut Vec<usize>,
) -> usize {
    if unused_adapters.len() == 0 {
        return 1;
    }

    let mut n_legal_successors = 0;
    while !unused_adapters.is_empty() && unused_adapters[0] <= adapters[adapters.len() - 1] + 3 {
        adapters.push(unused_adapters[0]);
        unused_adapters = &unused_adapters[1..];
        n_legal_successors +=
            count_legal_adapter_arrangements_memoized(adapters, unused_adapters, memoize);
        adapters.pop();
    }

    n_legal_successors
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let adapters: Vec<u32> = parse(input)?.collect();
    let stats = adapter_chain_stats(&adapters).ok_or(Error::SolutionNotFound)?;
    println!("stats: {:?}", stats);
    println!("1-diffs * 3-diffs = {}", stats[&1] * stats[&3]);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let adapters: Vec<u32> = parse(input)?.collect();
    let n_legal_arrangements = count_legal_adapter_arrangements(&adapters);
    println!("n legal adapter arrangements: {}", n_legal_arrangements);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("solution not found")]
    SolutionNotFound,
}

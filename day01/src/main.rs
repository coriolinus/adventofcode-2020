use aoc2020::config::Config;
use day01::{part1, part2};

use color_eyre::eyre::Result;
use structopt::StructOpt;
use std::path::PathBuf;

const DAY: u8 = 1;

#[derive(StructOpt, Debug)]
struct RunArgs {
    /// input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

    /// skip part 1
    #[structopt(long = "no-part1")]
    no_part1: bool,

    /// run part 2
    #[structopt(long)]
    part2: bool,
}

impl RunArgs {
    fn input(&self) -> Result<PathBuf> {
        match self.input {
            None => {
                let config = Config::load()?;
                Ok(config.input_for(DAY))
            }
            Some(ref path) => Ok(path.clone()),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = RunArgs::from_args();
    let input_path = args.input()?;

    if !args.no_part1 {
        part1(&input_path)?;
    }
    if args.part2 {
        part2(&input_path)?;
    }
    Ok(())
}

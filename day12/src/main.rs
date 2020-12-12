use aoc2020::{config::Config, website::get_input};
use day12::{part1, part2};

use color_eyre::eyre::Result;
use structopt::StructOpt;
use std::path::PathBuf;

const DAY: u8 = 12;

#[derive(StructOpt, Debug)]
struct RunArgs {
    /// input file
    #[structopt(long, parse(from_os_str))]
    input: Option<PathBuf>,

    /// skip part 1
    #[structopt(long)]
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
                // this does nothing if the input file already exists, but
                // simplifies the workflow after cloning the repo on a new computer
                get_input(&config, DAY)?;
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

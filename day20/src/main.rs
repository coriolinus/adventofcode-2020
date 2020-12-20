use aoc2020::{config::Config, website::get_input};
use day20::{part1, part2};

use color_eyre::eyre::Result;
use std::path::PathBuf;
use structopt::StructOpt;

const DAY: u8 = 20;

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

    let mut tiles_map = None;

    if !args.no_part1 {
        tiles_map = Some(part1(&input_path)?);
    }
    if args.part2 {
        if tiles_map.is_none() {
            tiles_map = Some(day20::tiles_map_from_input(&input_path)?);
        }
        part2(tiles_map.expect("it can't be none here; qed"))?;
    }
    Ok(())
}

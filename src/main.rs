use aoc2020::config::Config;
use chrono::{Datelike, Utc};
use color_eyre::eyre::{bail, Result};
use path_absolutize::Absolutize;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Copy, Debug)]
struct Day {
    /// Day (default: today's date)
    #[structopt(short, long)]
    day: Option<u8>,
}

impl From<Day> for u8 {
    fn from(day: Day) -> u8 {
        day.day.unwrap_or_else(|| Utc::now().day() as u8)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(about = "advent of code 2020")]
enum Subcommand {
    /// Manage configuration
    Config {
        #[structopt(subcommand)]
        cmd: ConfigOpts,
    },
    /// Emit the URL to a specified puzzle
    Url {
        #[structopt(flatten)]
        day: Day,
    },
    /// Initialize a puzzle
    Init {
        #[structopt(flatten)]
        day: Day,

        /// Do not create a sub-crate for the requested day
        #[structopt(long)]
        skip_create_crate: bool,

        /// Do not attempt to fetch the input for the requested day
        #[structopt(long)]
        skip_get_input: bool,
    },
}

impl Subcommand {
    fn run(self) -> Result<()> {
        match self {
            Self::Config { cmd } => cmd.run(),
            Self::Url { day } => {
                println!("{}", aoc2020::website::url_for_day(day.into()));
                Ok(())
            }
            Self::Init {
                day,
                skip_create_crate,
                skip_get_input,
            } => {
                let config = Config::load()?;
                aoc2020::day::initialize(&config, day.into(), skip_create_crate, skip_get_input)?;
                Ok(())
            }
        }
    }
}

#[derive(StructOpt, Debug)]
enum ConfigOpts {
    /// Emit the path to the configuration file
    Path,
    /// Display the contents of the configuration file, if they exist
    Show,
    /// Set configuration
    Set {
        /// Website session key
        ///
        /// Log in to adventofcode.com and inspect the cookies to get this
        #[structopt(short, long)]
        session: Option<String>,

        /// Path to input files
        #[structopt(short, long, parse(from_os_str))]
        inputs: Option<PathBuf>,
    },
}

impl ConfigOpts {
    fn run(self) -> Result<()> {
        match self {
            Self::Path => println!("{}", aoc2020::config::path().display()),
            Self::Show => {
                let data = std::fs::read_to_string(aoc2020::config::path())?;
                println!("{}", data);
            }
            Self::Set { session, inputs } => {
                let mut config = Config::load().unwrap_or_default();
                if let Some(session) = session {
                    if session.is_empty() {
                        bail!("session key must not be empty");
                    }
                    config.session = session;
                }
                if let Some(inputs) = inputs {
                    if inputs.exists() && !inputs.is_dir() {
                        bail!("inputs must be a directory");
                    }
                    config.input_files = Some(inputs.absolutize()?.into_owned());
                }
                config.save()?;
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Subcommand::from_args();
    opt.run()
}

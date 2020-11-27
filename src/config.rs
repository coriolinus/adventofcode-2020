use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

pub fn path() -> PathBuf {
    dirs::config_dir()
        .expect("advent of code must be run by a user with a home directory")
        .join("adventofcode")
        .join("2020.toml")
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// Session cookie
    pub session: String,

    /// Path to input files
    pub input_files: Option<PathBuf>,
}

impl Config {
    pub fn save(&self) -> Result<(), Error> {
        let path = path();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let serialized = toml::ser::to_string_pretty(self)?;
        std::fs::write(path, serialized.as_bytes()).map_err(Into::into)
    }

    pub fn load() -> Result<Self, Error> {
        let data = std::fs::read(path())?;
        toml::de::from_slice(&data).map_err(Into::into)
    }

    pub fn input_files(&self) -> PathBuf {
        match self.input_files {
            Some(ref input_files) => input_files.to_owned(),
            None => match std::env::current_dir() {
                Ok(current) => current.join("inputs"),
                Err(_) => dirs::data_dir()
                    .expect("advent of code must be run by a user with a home directory")
                    .join("adventofcode")
                    .join("2020"),
            },
        }
    }

    pub fn input_for(&self, day: u8) -> PathBuf {
        self.input_files().join(format!("input-{:02}.txt", day))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("configuration could not be loaded")]
    CouldNotLoad(#[from] std::io::Error),
    #[error("malformed configuration")]
    Malformed(#[from] toml::de::Error),
    #[error("failed to serialize")]
    CouldNotSerialize(#[from] toml::ser::Error),
}

use crate::config::Config;
use thiserror::Error;

/// Generate the puzzle URL for a given day
pub fn url_for_day(day: u8) -> String {
    format!("https://adventofcode.com/{}/day/{}", 2020, day)
}

/// Generate the input URL for a given day
pub fn input_url_for_day(day: u8) -> String {
    format!("{}/input", url_for_day(day))
}

/// Download the day's input file
///
/// If the file already exists, silently does nothing. This prevents server spam.
pub fn get_input(config: &Config, day: u8) -> Result<(), Error> {
    let input_path = config.input_for(day);
    if input_path.exists() {
        return Ok(());
    }

    let client = reqwest::blocking::Client::builder()
        .gzip(true)
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(Error::ClientBuilder)?;

    let mut response = client
        .get(&input_url_for_day(day))
        .header(
            reqwest::header::COOKIE,
            format!("session={}", config.session),
        )
        .send()
        .map_err(Error::RequestingInput)?
        .error_for_status()
        .map_err(Error::ResponseStatus)?;

    if let Some(parent) = input_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let mut file = std::fs::File::create(input_path)?;
    response.copy_to(&mut file).map_err(Error::Downloading)?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("building request client")]
    ClientBuilder(#[source] reqwest::Error),
    #[error("requesting input file")]
    RequestingInput(#[source] reqwest::Error),
    #[error("response status unsuccessful")]
    ResponseStatus(#[source] reqwest::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("downloading to local file")]
    Downloading(#[source] reqwest::Error),
}

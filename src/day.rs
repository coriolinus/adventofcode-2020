use cargo_toml::Manifest;
use serde::Serialize;
use thiserror::Error;
use tinytemplate::TinyTemplate;

use crate::config::Config;

const EXPECT_PACKAGE: &str = env!("CARGO_PKG_NAME");

#[derive(Serialize)]
struct Context {
    day: u8,
    package_name: String,
}

/// Initialize a new day.
///
/// This entails:
///
/// - ensuring we're in the right crate
/// - creating a new sub-crate
/// - updating the workspaces of this crate
/// - copying in a few templates to set up the day
/// - downloading the puzzle input
pub fn initialize(config: &Config, day: u8) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    // parse the local Cargo.toml to discover if we're in the right place
    let cargo_toml_path = current_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        Err(Error::NoCargoToml)?;
    }
    let mut manifest = Manifest::from_path(&cargo_toml_path)?;
    let found_package_name = manifest
        .package
        .as_ref()
        .ok_or(Error::NoPackage)?
        .name
        .clone();
    if found_package_name != EXPECT_PACKAGE {
        Err(Error::WrongPackage(found_package_name))?;
    }

    // create a new sub-crate
    let day_name = format!("day{:02}", day);
    let day_dir = current_dir.join(&day_name);
    std::fs::create_dir_all(day_dir.join("src"))?;

    // update the workspaces of this crate
    let mut workspace = manifest.workspace.unwrap_or_default();
    workspace.members.push(day_name.clone());
    manifest.workspace = Some(workspace);
    let serialized = toml::ser::to_string_pretty(&manifest)?;
    std::fs::write(cargo_toml_path, serialized.as_bytes())?;

    // set up the templates
    let context = Context {
        day,
        package_name: day_name,
    };

    // render templates
    let template_dir = current_dir.join("day-template");
    for template in &["Cargo.toml", "src/lib.rs", "src/main.rs"] {
        let mut tt = TinyTemplate::new();
        let template_text = std::fs::read_to_string(template_dir.join(template))?;
        tt.add_template(template, &template_text)?;
        let rendered_text = tt.render(template, &context)?;
        std::fs::write(day_dir.join(template), rendered_text)?;
    }

    // download the input
    crate::website::get_input(config, day)?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Cargo.toml not found")]
    NoCargoToml,
    #[error(transparent)]
    CargoToml(#[from] cargo_toml::Error),
    #[error("Cargo.toml did not contain section: [package]")]
    NoPackage,
    #[error(
        "working dir must be root of package {} but is actually {0}",
        EXPECT_PACKAGE
    )]
    WrongPackage(String),
    #[error("failed to write updated Cargo.toml")]
    CargoTomlWrite(#[from] toml::ser::Error),
    #[error(transparent)]
    Template(#[from] tinytemplate::error::Error),
    #[error("downloading input")]
    GetInput(#[from] crate::website::Error),
}

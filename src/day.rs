use serde::Serialize;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use thiserror::Error;
use tinytemplate::TinyTemplate;
use toml_edit::Document;

use crate::config::Config;

const EXPECT_PACKAGE: &str = env!("CARGO_PKG_NAME");

/// ensure we're in the correct directory by verifying the package name in `Cargo.toml`
fn ensure_correct_dir(current_dir: &Path) -> Result<(PathBuf, Document), Error> {
    // parse the local Cargo.toml to discover if we're in the right place
    let cargo_toml_path = current_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        Err(Error::NoCargoToml)?;
    }
    let manifest = Document::from_str(&std::fs::read_to_string(&cargo_toml_path)?)?;

    fn get_package_name(manifest: &Document) -> Option<&str> {
        manifest
            .root
            .as_table()
            .expect("document root is a table")
            .get("package")?
            .as_table_like()?
            .get("name")?
            .as_value()?
            .as_str()
    }

    let found_package_name = get_package_name(&manifest).ok_or(Error::MalformedToml)?;

    if found_package_name != EXPECT_PACKAGE {
        Err(Error::WrongPackage(found_package_name.to_string()))?;
    }
    Ok((cargo_toml_path, manifest))
}

fn add_crate_to_workspace(
    cargo_toml_path: &Path,
    manifest: &mut Document,
    crate_name: &str,
) -> Result<(), Error> {
    let root_table = manifest
        .root
        .as_table_mut()
        .expect("docuemnt root is a table");

    let workspace = root_table.entry("workspace");
    if workspace.is_none() {
        *workspace = toml_edit::Item::Table(toml_edit::Table::new());
    }
    let workspace = workspace.as_table_mut().ok_or(Error::MalformedToml)?;

    let members = workspace.entry("members");
    if members.is_none() {
        *members = toml_edit::Item::Value(toml_edit::Value::Array(Default::default()));
    }
    let members = members
        .as_value_mut()
        .ok_or(Error::MalformedToml)?
        .as_array_mut()
        .ok_or(Error::MalformedToml)?;

    if members.iter().any(|item| {
        item.as_str()
            .map(|item_str| item_str == crate_name)
            .unwrap_or_default()
    }) {
        Err(Error::CrateAlreadyExists(crate_name.to_string()))?;
    }

    members.push(crate_name).map_err(|_| Error::MalformedToml)?;

    std::fs::write(cargo_toml_path, manifest.to_string_in_original_order())?;
    Ok(())
}

fn render_templates_into(
    current_dir: &Path,
    day_dir: &Path,
    day: u8,
    day_name: &str,
) -> Result<(), Error> {
    use std::io::Write;

    #[derive(Serialize)]
    struct Context {
        day: u8,
        package_name: String,
    }

    let context = Context {
        day,
        package_name: day_name.to_string(),
    };

    // render templates
    let template_dir = current_dir.join("day-template");
    for template in &["Cargo.toml", "src/lib.rs", "src/main.rs"] {
        let mut tt = TinyTemplate::new();
        let template_text = std::fs::read_to_string(template_dir.join(template))?;
        tt.add_template(template, &template_text)
            .map_err(|err| Error::Template(err, template.to_string()))?;
        let rendered_text = tt
            .render(template, &context)
            .map_err(|err| Error::Template(err, template.to_string()))?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(day_dir.join(template))?;
        file.write_all(rendered_text.as_bytes())?;
    }

    Ok(())
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
pub fn initialize(
    config: &Config,
    day: u8,
    skip_create_crate: bool,
    skip_get_input: bool,
) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let (cargo_toml_path, mut manifest) = ensure_correct_dir(&current_dir)?;

    if !skip_create_crate {
        // set up new sub-crate basics
        let day_name = format!("day{:02}", day);
        let day_dir = current_dir.join(&day_name);
        std::fs::create_dir_all(day_dir.join("src"))?;

        // update the workspaces of this crate
        add_crate_to_workspace(&cargo_toml_path, &mut manifest, &day_name)?;

        // render templates, creating new sub-crate
        render_templates_into(&current_dir, &day_dir, day, &day_name)?;
    }

    if !skip_get_input {
        // download the input
        crate::website::get_input(config, day)?;
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Cargo.toml not found")]
    NoCargoToml,
    #[error("could not parse Cargo.toml")]
    ParseToml(#[from] toml_edit::TomlError),
    #[error("Cargo.toml is malformed")]
    MalformedToml,
    #[error(
        "working dir must be root of package {} but is actually {0}",
        EXPECT_PACKAGE
    )]
    WrongPackage(String),
    #[error("failed to write updated Cargo.toml")]
    CargoTomlWrite(#[from] toml::ser::Error),
    #[error("template error for {1}")]
    Template(#[source] tinytemplate::error::Error, String),
    #[error("downloading input")]
    GetInput(#[from] crate::website::Error),
    #[error("crate already exists in workspace: {0}")]
    CrateAlreadyExists(String),
}

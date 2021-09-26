use std::path::Path;

use anyhow::{Context, Result};

use crate::{fs, logger, package::Package};

pub fn init(name: Option<String>) -> Result<String> {
    // Use provided name
    // If the name is not provided attempt to use the cwd name
    // Else, set name as empty string, should we just throw an error?
    let name = name.or(fs::cwd());
    let package = Package {
        name,
        ..Package::default()
    };
    let package = package.to_json().context("Serializing package.json")?;
    let path = Path::new("package.json");
    fs::echo(&package, path).context("Creating package.json")?;

    Ok(logger::success("Saved package.json"))
}

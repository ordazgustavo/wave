use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::{definitions::Package, logger, utils, WaveContext};

pub struct InitFlags {
    pub yes: bool,
}

pub fn init(ctx: &WaveContext, name: Option<String>, flags: &InitFlags) -> Result<()> {
    // Use provided name
    // If the name is not provided attempt to use the cwd name
    // Else, set name as empty string, should we just throw an error?
    if flags.yes {
        logger::warning(ctx, "The yes flag has been set. This will automatically answer yes to all questions, which may have security implications.")?;
    }
    let name = name.or(Some(utils::cwd()?));
    let package = Package {
        name,
        ..Package::default()
    };
    let package = package.to_json().context("Serializing package.json")?;
    let path = Path::new("package.json");
    fs::write(path, package).context("Creating package.json")?;

    logger::success(ctx, "Saved package.json")
}

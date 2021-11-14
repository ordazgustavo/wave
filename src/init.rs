use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use console::Term;

use crate::{fs, logger, package::Package};

pub struct InitFlags {
    pub yes: bool,
}

pub fn init(term: &mut Term, name: Option<String>, flags: InitFlags) -> Result<String> {
    // Use provided name
    // If the name is not provided attempt to use the cwd name
    // Else, set name as empty string, should we just throw an error?
    if flags.yes {
        writeln!(term, "{}", logger::warning("The yes flag has been set. This will automatically answer yes to all questions, which may have security implications."))?;
    }
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

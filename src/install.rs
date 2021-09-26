use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};

use crate::{fs, logger, package::Package};

pub fn install(packages: Vec<String>) -> Result<String> {
    let package_path = Path::new("package.json");
    let package = fs::cat(package_path).context("Reading package.json")?;
    let package = Package::from_json(&package).context("Deserializing package.json")?;

    let mut map = HashMap::<String, String>::new();
    for key in packages.into_iter() {
        map.insert(key, "latest".to_string());
    }
    if let Some(prev_deps) = package.dependencies {
        map.extend(prev_deps);
    }
    let new_package = Package {
        dependencies: Some(map),
        ..package
    };
    let package_json = new_package.to_json().context("Serializing package.json")?;
    fs::echo(&package_json, package_path).context("Updating package.json")?;

    Ok(logger::success("Saved package.json"))
}

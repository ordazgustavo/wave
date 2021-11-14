use std::{collections::BTreeMap, path::Path};

use anyhow::{Context, Result};

use crate::{fs, logger, package::Package};

pub struct InstallFlags {
    pub development: bool,
    pub exact: bool,
}

pub fn install(packages: Vec<String>, flags: InstallFlags) -> Result<String> {
    let package_path = Path::new("package.json");
    let package = fs::cat(package_path).context("Reading package.json")?;
    let mut package = Package::from_json(&package).context("Deserializing package.json")?;

    let mut map = BTreeMap::<String, String>::new();
    for key in packages.into_iter() {
        map.insert(key, "latest".to_string());
    }

    if flags.development {
        if let Some(prev_deps) = package.dev_dependencies {
            map.extend(prev_deps);
        }
        package.dev_dependencies = Some(map);
    } else {
        if let Some(prev_deps) = package.dependencies {
            map.extend(prev_deps);
        }
        package.dependencies = Some(map);
    }

    let package_json = package.to_json().context("Serializing package.json")?;
    fs::echo(&package_json, package_path).context("Updating package.json")?;

    Ok(logger::success("Saved package.json"))
}

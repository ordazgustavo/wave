use std::{collections::BTreeMap, path::Path};

use anyhow::Result;

use crate::{fs, logger, package::Package, registry, WaveContext};

pub struct InstallFlags {
    pub development: bool,
    pub exact: bool,
}

/// Handles packages map generation with correct versions
async fn generate_packages_map(
    ctx: &WaveContext,
    packages: Vec<String>,
) -> Result<BTreeMap<String, String>> {
    let mut map = BTreeMap::<String, String>::new();
    for key in packages.into_iter() {
        let k: Vec<_> = key.split('@').collect();
        let package_name = k[0];
        let version =
            registry::get_package_version(&ctx, package_name, *k.get(1).unwrap_or(&"latest"))
                .await?;
        map.insert(String::from(package_name), version);
    }
    Ok(map)
}

pub async fn install(ctx: &WaveContext, packages: Vec<String>, flags: InstallFlags) -> Result<()> {
    let package_path = Path::new("package.json");
    let package = fs::cat(package_path)?;
    let mut package = Package::from_json(&package)?;

    let map = generate_packages_map(&ctx, packages).await?;

    if flags.development {
        if let Some(mut prev_deps) = package.dev_dependencies {
            prev_deps.extend(map);
            package.dev_dependencies = Some(prev_deps);
        } else {
            package.dev_dependencies = Some(map);
        }
    } else {
        if let Some(mut prev_deps) = package.dependencies {
            prev_deps.extend(map);
            package.dependencies = Some(prev_deps);
        } else {
            package.dependencies = Some(map);
        }
    }

    let package_json = package.to_json()?;
    fs::echo(&package_json, package_path)?;

    logger::success(&ctx, "Saved package.json")
}

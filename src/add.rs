use std::{collections::BTreeMap, path::Path};

use anyhow::Result;

use crate::{
    definitions::Package,
    fs::{cat, echo},
    logger, utils, WaveContext,
};

pub struct AddFlags {
    pub development: bool,
    pub exact: bool,
}

pub async fn add(
    ctx: &WaveContext,
    packages: Vec<(String, String)>,
    flags: AddFlags,
) -> Result<()> {
    let mut installed_deps = Vec::new();
    for (name, version) in packages.into_iter() {
        installed_deps.push(utils::get_dependency_tree(&ctx, &name, &version).await?);
    }

    let package_path = Path::new("package.json");
    let package = cat(package_path)?;
    let mut package = Package::from_json(&package)?;
    let updated_versions = installed_deps.iter().fold(BTreeMap::new(), |mut acc, x| {
        acc.insert(x.name.clone(), x.version.clone());
        acc
    });

    if flags.development {
        package.dev_dependencies = match package.dev_dependencies {
            Some(mut prev) => {
                prev.extend(updated_versions);
                Some(prev)
            }
            None => Some(updated_versions),
        }
    } else {
        package.dependencies = match package.dependencies {
            Some(mut prev) => {
                prev.extend(updated_versions);
                Some(prev)
            }
            None => Some(updated_versions),
        }
    }

    let package_json = package.to_json()?;
    echo(&package_json, package_path)?;

    let resolved_packages =
        utils::flatten_deps(&installed_deps.iter().map(|d| Box::new(d.clone())).collect());
    utils::update_node_modules(&ctx, &resolved_packages).await?;
    utils::save_lockfile(resolved_packages)?;

    logger::success(&ctx, "Saved package.json")
}

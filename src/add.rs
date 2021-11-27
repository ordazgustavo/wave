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
    let mut updated_versions = BTreeMap::new();
    for (name, version) in packages.into_iter() {
        let installed_version = utils::get_dependency_tree(&ctx, &name, &version).await?;
        updated_versions.insert(name, installed_version);
    }

    let package_path = Path::new("package.json");
    let package = cat(package_path)?;
    let mut package = Package::from_json(&package)?;

    if flags.development {
        if let Some(mut prev_deps) = package.dev_dependencies {
            prev_deps.extend(updated_versions);
            package.dev_dependencies = Some(prev_deps);
        } else {
            package.dev_dependencies = Some(updated_versions);
        }
    } else {
        if let Some(mut prev_deps) = package.dependencies {
            prev_deps.extend(updated_versions);
            package.dependencies = Some(prev_deps);
        } else {
            package.dependencies = Some(updated_versions);
        }
    }

    let package_json = package.to_json()?;
    echo(&package_json, package_path)?;

    logger::success(&ctx, "Saved package.json")
}

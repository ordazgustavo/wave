use std::fs;

use anyhow::Result;

use crate::{definitions::Package, utils, WaveContext};

pub async fn install(ctx: &WaveContext) -> Result<()> {
    let package = fs::read_to_string("package.json")?;
    let package = Package::from_json(&package)?;

    let mut installed_deps = Vec::new();
    if let Some(dependencies) = package.dependencies {
        for (name, version) in dependencies {
            installed_deps.push(utils::get_dependency_tree(ctx, &name, &version).await?);
        }
    }

    if let Some(dependencies) = package.dev_dependencies {
        for (name, version) in dependencies {
            installed_deps.push(utils::get_dependency_tree(ctx, &name, &version).await?);
        }
    }

    let resolved_packages = utils::flatten_deps(&installed_deps);
    utils::update_node_modules(ctx, &resolved_packages).await?;
    utils::save_lockfile(resolved_packages)?;

    Ok(())
}

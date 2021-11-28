use std::path::Path;

use anyhow::Result;

use crate::{definitions::Package, fs::cat, utils, WaveContext};

pub async fn install(ctx: &WaveContext) -> Result<()> {
    let package_path = Path::new("package.json");
    let package = cat(package_path)?;
    let package = Package::from_json(&package)?;

    let mut installed_deps = Vec::new();
    if let Some(dependencies) = package.dependencies {
        for (name, version) in dependencies.into_iter() {
            installed_deps.push(utils::get_dependency_tree(&ctx, &name, &version).await?);
        }
    }

    if let Some(dependencies) = package.dev_dependencies {
        for (name, version) in dependencies.into_iter() {
            installed_deps.push(utils::get_dependency_tree(&ctx, &name, &version).await?);
        }
    }

    let resolved_packages =
        utils::flatten_deps(&installed_deps.iter().map(|d| Box::new(d.clone())).collect());
    utils::update_node_modules(&ctx, &resolved_packages).await?;
    utils::save_lockfile(resolved_packages)?;

    Ok(())
}
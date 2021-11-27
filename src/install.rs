use std::path::Path;

use anyhow::Result;

use crate::{definitions::Package, fs::cat, utils, WaveContext};

pub async fn install(ctx: &WaveContext) -> Result<()> {
    let package_path = Path::new("package.json");
    let package = cat(package_path)?;
    let package = Package::from_json(&package)?;

    if let Some(dependencies) = package.dependencies {
        for (name, version) in dependencies.into_iter() {
            utils::get_dependency_tree(&ctx, &name, &version).await?;
        }
    }

    if let Some(dependencies) = package.dev_dependencies {
        for (name, version) in dependencies.into_iter() {
            utils::get_dependency_tree(&ctx, &name, &version).await?;
        }
    }

    Ok(())
}

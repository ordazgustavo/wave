use std::path::Path;

use anyhow::Result;

use crate::{fs::cat, package::Package, registry, utils, WaveContext};

pub async fn install(ctx: &WaveContext) -> Result<()> {
    let package_path = Path::new("package.json");
    let package = cat(package_path)?;
    let package = Package::from_json(&package)?;

    if let Some(dependencies) = package.dependencies {
        for (name, version) in dependencies.into_iter() {
            let packument = registry::get_package_data(&ctx, &name, &version).await?;
            let bytes = utils::get_package_tarball(&ctx, &packument.dist.tarball).await?;
            let mut archive = utils::decode_tarball(bytes);
            utils::save_package_in_node_modules(&name, &mut archive)?;
        }
    }

    if let Some(dependencies) = package.dev_dependencies {
        for (name, version) in dependencies.into_iter() {
            let packument = registry::get_package_data(&ctx, &name, &version).await?;
            let bytes = utils::get_package_tarball(&ctx, &packument.dist.tarball).await?;
            let mut archive = utils::decode_tarball(bytes);
            utils::save_package_in_node_modules(&name, &mut archive)?;
        }
    }

    Ok(())
}

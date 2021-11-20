use std::{collections::BTreeMap, fs, path::Path};

use anyhow::Result;
use bytes::Buf;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::{
    fs::{cat, echo},
    logger,
    package::Package,
    registry, WaveContext,
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
    let package_path = Path::new("package.json");
    let package = cat(package_path)?;
    let mut package = Package::from_json(&package)?;

    let mut updated_versions = BTreeMap::new();
    for (name, version) in packages.into_iter() {
        let packument = registry::get_package_data(&ctx, &name, &version).await?;
        updated_versions.insert(name, packument.version);
        let bytes = ctx
            .client
            .get(packument.dist.tarball)
            .send()
            .await?
            .bytes()
            .await?;
        let tar = GzDecoder::new(bytes.reader());
        let mut archive = Archive::new(tar);

        let prefix = "package";
        let node_modules = format!("node_modules/{}", packument.name);
        let dest = Path::new(&node_modules);
        fs::create_dir_all(dest)?;

        for mut entry in archive.entries()?.filter_map(|e| e.ok()).into_iter() {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            if let Some(parent) = path.parent() {
                let nm_inner_dir = Path::new(&node_modules).join(parent);
                if !nm_inner_dir.exists() {
                    fs::create_dir_all(nm_inner_dir)?;
                }
            }
            entry.unpack(dest.join(path))?;
        }
    }

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

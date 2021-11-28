use std::fs;
use std::path::Path;

use anyhow::Result;
use async_recursion::async_recursion;
use bytes::{buf::Reader, Buf, Bytes};
use flate2::read::GzDecoder;
use node_semver::{Range, Version};
use tar::Archive;

use crate::{registry, WaveContext};

pub fn decode_tarball(bytes: Bytes) -> Archive<GzDecoder<Reader<Bytes>>> {
    let tar = GzDecoder::new(bytes.reader());
    Archive::new(tar)
}

pub async fn get_package_tarball(ctx: &WaveContext, url: &str) -> Result<Bytes> {
    Ok(ctx.client.get(url).send().await?.bytes().await?)
}

pub fn save_package_in_node_modules<T>(name: &str, archive: &mut Archive<T>) -> Result<()>
where
    T: std::io::Read,
{
    // The folder name of the unpacked tarball
    let prefix = "package";
    let node_modules = format!("node_modules/{}", name);
    let dest = Path::new(&node_modules);
    // Remove the previous module if it already exists
    if dest.exists() {
        fs::remove_dir_all(dest)?;
    }
    fs::create_dir_all(dest)?;

    for mut entry in archive.entries()?.filter_map(|e| e.ok()).into_iter() {
        let path = entry.path()?.strip_prefix(prefix)?.to_owned();
        let path = dest.join(path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        entry.unpack(path)?;
    }

    Ok(())
}

/// Returns the installed version
#[async_recursion]
pub async fn get_dependency_tree(ctx: &WaveContext, name: &str, version: &str) -> Result<String> {
    let packument = registry::get_package_document(&ctx, &name).await?;
    let version = version.to_string();
    let version = packument.dist_tags.get(&version).unwrap_or(&version);
    let version: Range = version.parse()?;
    let version = packument
        .versions
        .keys()
        .filter(|v| version.satisfies(&v.parse().expect("")))
        .max_by(|a, b| {
            let a = a.parse::<Version>().expect("");
            let b = b.parse::<Version>().expect("");
            a.cmp(&b)
        });

    if let Some(version) = version {
        let package_metadata = packument.versions.get(version);
        if let Some(package_metadata) = package_metadata {
            let bytes = get_package_tarball(&ctx, &package_metadata.dist.tarball).await?;
            let mut archive = decode_tarball(bytes);
            save_package_in_node_modules(&name, &mut archive)?;
            if let Some(dependencies) = &package_metadata.dependencies {
                for (name, version) in dependencies {
                    get_dependency_tree(&ctx, &name, &version).await?;
                }
            }
            Ok(package_metadata.version.clone())
        } else {
            anyhow::bail!("Couldn't get package metadata");
        }
    } else {
        anyhow::bail!("Couldn't get package metadata");
    }
}

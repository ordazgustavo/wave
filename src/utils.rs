use std::path::Path;
use std::{collections::BTreeMap, fs};

use anyhow::Result;
use async_recursion::async_recursion;
use bytes::{buf::Reader, Buf, Bytes};
use flate2::read::GzDecoder;
use node_semver::{Range, Version};
use tar::Archive;

use crate::{
    definitions::{ResolvedPackage, ResolvedPackages, WaveLockfile},
    fs::{cat, echo},
    registry, WaveContext,
};

#[derive(Debug, Clone)]
pub struct DependencyTree {
    pub name: String,
    pub version: String,
    pub resolved: String,
    pub integrity: Option<String>,
    pub dependencies: Vec<Box<DependencyTree>>,
}

#[async_recursion]
pub async fn get_dependency_tree(
    ctx: &WaveContext,
    name: &str,
    version: &str,
) -> Result<DependencyTree> {
    let packument = registry::get_package_document(&ctx, &name).await?;
    let version = version.to_string();
    let version = packument.dist_tags.get(&version).unwrap_or(&version);
    let version: Range = version.parse()?;
    let version = packument
        .versions
        .keys()
        .filter(|v| version.satisfies(&v.parse().expect("")))
        .max_by(|a, b| {
            let a: Version = a.parse().expect("");
            let b: Version = b.parse().expect("");
            a.cmp(&b)
        });

    match version {
        Some(version) => match packument.versions.get(version) {
            Some(package_metadata) => {
                let mut deps = Vec::new();
                if let Some(dependencies) = &package_metadata.dependencies {
                    for (name, version) in dependencies {
                        deps.push(Box::new(get_dependency_tree(&ctx, &name, &version).await?));
                    }
                }
                Ok(DependencyTree {
                    name: package_metadata.name.clone(),
                    version: package_metadata.version.clone(),
                    resolved: package_metadata.dist.tarball.clone(),
                    integrity: package_metadata.dist.integrity.clone(),
                    dependencies: deps,
                })
            }
            None => anyhow::bail!("Couldn't get package metadata"),
        },
        None => anyhow::bail!("Couldn't get package metadata"),
    }
}

pub fn save_lockfile(resolved_packages: ResolvedPackages) -> Result<()> {
    let path = WaveLockfile::location();
    let lockfile = if !WaveLockfile::is_defined() {
        WaveLockfile::new(resolved_packages)
    } else {
        let lockfile = cat(path)?;
        let mut lockfile = WaveLockfile::from_json(&lockfile)?;
        lockfile.packages.extend(resolved_packages);
        lockfile
    };
    echo(&lockfile.to_json()?, path)?;
    Ok(())
}

pub fn flatten_deps(dependency_tree: &Vec<Box<DependencyTree>>) -> ResolvedPackages {
    dependency_tree.iter().fold(BTreeMap::new(), |mut acc, x| {
        acc.insert(
            x.name.clone(),
            ResolvedPackage {
                version: x.version.clone(),
                resolved: x.resolved.clone(),
                integrity: x.integrity.clone(),
                dependencies: if x.dependencies.len() > 0 {
                    Some(x.dependencies.iter().map(|d| d.name.clone()).collect())
                } else {
                    None
                },
            },
        );
        let nested = flatten_deps(&x.dependencies);
        acc.extend(nested);
        acc
    })
}

pub async fn update_node_modules(
    ctx: &WaveContext,
    resolved_packages: &ResolvedPackages,
) -> Result<()> {
    for (name, pkg) in resolved_packages.iter() {
        let bytes = get_package_tarball(&ctx, &pkg.resolved).await?;
        let mut archive = decode_tarball(bytes);
        save_package_in_node_modules(&name, &mut archive).expect("");
    }
    Ok(())
}

pub async fn get_package_tarball(ctx: &WaveContext, url: &str) -> Result<Bytes> {
    Ok(ctx.client.get(url).send().await?.bytes().await?)
}

pub fn decode_tarball(bytes: Bytes) -> Archive<GzDecoder<Reader<Bytes>>> {
    let tar = GzDecoder::new(bytes.reader());
    Archive::new(tar)
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

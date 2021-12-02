use std::{
    collections::{BTreeMap, HashMap},
    env, fs,
    path::Path,
};

use anyhow::{Context, Result};
use async_recursion::async_recursion;
use bytes::{buf::Reader, Buf, Bytes};
use flate2::read::GzDecoder;
use node_semver::{Range, Version};
use tar::Archive;

use crate::{
    definitions::{ResolvedPackage, ResolvedPackages, WaveLockfile},
    registry, WaveContext,
};

#[derive(Debug, Clone)]
pub struct DependencyTree {
    pub name: String,
    pub version: String,
    pub resolved: String,
    pub integrity: Option<String>,
    pub symlinks: Option<HashMap<String, String>>,
    pub dependencies: Vec<DependencyTree>,
}

#[async_recursion]
pub async fn get_dependency_tree(
    ctx: &WaveContext,
    name: &str,
    version: &str,
) -> Result<DependencyTree> {
    let lockfile = WaveLockfile::read();
    let locked_package = lockfile.and_then(|lockfile| lockfile.get_package(name, version));

    if let Some(pkg) = locked_package {
        let mut deps = Vec::new();
        if let Some(dependencies) = pkg.dependencies {
            for (name, version) in dependencies {
                deps.push(get_dependency_tree(ctx, &name, &version).await?);
            }
        }
        Ok(DependencyTree {
            name: name.to_owned(),
            version: pkg.version,
            resolved: pkg.resolved,
            integrity: pkg.integrity,
            symlinks: None,
            dependencies: deps,
        })
    } else {
        let packument = registry::get_package_document(ctx, name).await?;
        let version = version.to_owned();
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
                            deps.push(get_dependency_tree(ctx, name, version).await?);
                        }
                    }
                    Ok(DependencyTree {
                        name: package_metadata.name.clone(),
                        version: package_metadata.version.clone(),
                        resolved: package_metadata.dist.tarball.clone(),
                        integrity: package_metadata.dist.integrity.clone(),
                        symlinks: package_metadata.bin.clone(),
                        dependencies: deps,
                    })
                }
                None => anyhow::bail!("Couldn't get package metadata"),
            },
            None => anyhow::bail!("Couldn't get package metadata"),
        }
    }
}

pub fn save_lockfile(resolved_packages: ResolvedPackages) -> Result<()> {
    let path = WaveLockfile::location();
    let lockfile = if WaveLockfile::is_defined() {
        let lockfile = fs::read_to_string(path)?;
        let mut lockfile = WaveLockfile::from_json(&lockfile)?;
        lockfile.packages.extend(resolved_packages);
        lockfile
    } else {
        WaveLockfile::new(resolved_packages)
    };
    fs::write(path, lockfile.to_json()?)?;
    Ok(())
}

pub fn flatten_deps(dependency_tree: &[DependencyTree]) -> ResolvedPackages {
    dependency_tree.iter().fold(BTreeMap::new(), |mut acc, x| {
        acc.insert(
            x.name.clone(),
            ResolvedPackage {
                version: x.version.clone(),
                resolved: x.resolved.clone(),
                integrity: x.integrity.clone(),
                dependencies: if x.dependencies.is_empty() {
                    None
                } else {
                    Some(x.dependencies.iter().fold(BTreeMap::new(), |mut acc, d| {
                        acc.insert(d.name.clone(), d.version.clone());
                        acc
                    }))
                },
            },
        );
        let nested = flatten_deps(&x.dependencies);
        acc.extend(nested);
        acc
    })
}

#[async_recursion]
pub async fn update_node_modules(
    ctx: &WaveContext,
    dependency_tree: &[DependencyTree],
) -> Result<()> {
    for pkg in dependency_tree {
        if !pkg.dependencies.is_empty() {
            update_node_modules(ctx, &pkg.dependencies).await?;
        }
        let bytes = get_package_tarball(ctx, &pkg.resolved).await?;
        let mut archive = decode_tarball(bytes);
        save_package_in_node_modules(&pkg.name, &mut archive, &pkg.symlinks)?;
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

pub fn save_package_in_node_modules<T>(
    name: &str,
    archive: &mut Archive<T>,
    symlinks: &Option<HashMap<String, String>>,
) -> Result<()>
where
    T: std::io::Read,
{
    // The folder name of the unpacked tarball
    let prefix = "package";
    let node_modules = format!("node_modules/{}", name);
    let bin_folder = Path::new("node_modules/.bin");
    let dest = Path::new(&node_modules);
    let has_symlinks = symlinks.is_some();

    // First remove previous symlinks
    if has_symlinks {
        for k in symlinks.as_ref().unwrap().keys() {
            let file = bin_folder.join(k);
            file.exists().then(|| fs::remove_file(file));
        }
    }

    // Remove the previous module if it already exists
    if dest.exists() {
        fs::remove_dir_all(dest)?;
    }

    // Create package node_modules destination
    fs::create_dir_all(dest)?;

    for mut entry in archive.entries()?.filter_map(|e| e.ok()) {
        let path = entry.path()?.strip_prefix(prefix)?.to_owned();
        let path = dest.join(path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        entry.unpack(path)?;
    }

    // Create .bin folder
    if !bin_folder.exists() {
        fs::create_dir_all(bin_folder)?;
    }

    // Recreate symlinks
    if has_symlinks {
        for (k, v) in symlinks.as_ref().unwrap() {
            let origin = pathdiff::diff_paths(dest.join(v), bin_folder);
            let destination = bin_folder.join(k);
            if let Some(origin) = origin {
                if !destination.exists() {
                    symlink::symlink_file(origin, destination).context("Creating symlink")?;
                }
            }
        }
    }

    Ok(())
}

pub fn cwd() -> Result<String> {
    let path = env::current_dir().context("Couldn't get cwd")?;
    let dir = path.file_name().context("Couldn't get cwd")?;
    Ok(dir.to_str().context("Couldn't get cwd")?.to_owned())
}

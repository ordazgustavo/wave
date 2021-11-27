use anyhow::{Context, Result};

use crate::{
    definitions::{PackageMetadata, Packument},
    WaveContext,
};

static ENDPOINT: &'static str = "https://registry.npmjs.org";

fn generate_url(name: &str, version: &str) -> String {
    format!("{}/{}/{}", ENDPOINT, name, version)
}

pub async fn get_package_data(
    ctx: &WaveContext,
    name: &str,
    version: &str,
) -> Result<PackageMetadata> {
    // Semver symbols
    let semver_symbols: &[_] = &['^', '~'];
    let package = ctx
        .client
        .get(generate_url(
            name,
            version.trim_start_matches(semver_symbols),
        ))
        .send()
        .await?
        .json::<PackageMetadata>()
        .await
        .context("Decoding package metadata")?;
    Ok(package)
}

pub async fn get_package_document(ctx: &WaveContext, name: &str) -> Result<Packument> {
    let package = ctx
        .client
        .get(format!("{}/{}", ENDPOINT, name))
        .header("Accept", "application/vnd.npm.install-v1+json")
        .send()
        .await?
        .json::<Packument>()
        .await
        .context("Decoding packument")?;
    Ok(package)
}

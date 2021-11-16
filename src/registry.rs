use anyhow::Result;

use crate::{package::Package, WaveContext};

static ENDPOINT: &'static str = "https://registry.npmjs.org";

fn generate_url(name: &str, version: &str) -> String {
    format!("{}/{}/{}", ENDPOINT, name, version)
}

pub async fn get_package_data(ctx: &WaveContext, name: &str, version: &str) -> Result<Package> {
    let package = ctx
        .client
        .get(generate_url(name, version))
        .send()
        .await?
        .json::<Package>()
        .await?;
    Ok(package)
}

pub async fn get_package_version(ctx: &WaveContext, name: &str, version: &str) -> Result<String> {
    let package = get_package_data(&ctx, name, version).await?;
    Ok(package.version.unwrap())
}

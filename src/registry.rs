use anyhow::{Context, Result};

use crate::{definitions::Packument, WaveContext};

static ENDPOINT: &str = "https://registry.npmjs.org";

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

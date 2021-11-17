use anyhow::Result;

use crate::{packument::Packument, WaveContext};

static ENDPOINT: &'static str = "https://registry.npmjs.org";

fn generate_url(name: &str, version: &str) -> String {
    format!("{}/{}/{}", ENDPOINT, name, version)
}

pub async fn get_package_data(ctx: &WaveContext, name: &str, version: &str) -> Result<Packument> {
    let package = ctx
        .client
        .get(generate_url(name, version))
        .send()
        .await?
        .json::<Packument>()
        .await?;
    Ok(package)
}

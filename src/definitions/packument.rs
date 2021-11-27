use std::collections::HashMap;

use serde::Deserialize;

use super::package_metadata::PackageMetadata;

#[derive(Deserialize, Debug)]
pub struct Packument {
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, String>,
    pub versions: HashMap<String, PackageMetadata>,
}

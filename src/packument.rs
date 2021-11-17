use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Packument {
    pub name: String,
    pub version: String,
    pub dist: Dist,
}

#[derive(Deserialize, Debug)]
pub struct Dist {
    pub tarball: String,
    pub shasum: String,
    pub integrity: String,
}

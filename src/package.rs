use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub main: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<BTreeMap<String, String>>,
}

impl Package {
    pub fn from_json(pkg: &str) -> Result<Self> {
        serde_json::from_str(pkg)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for Package {
    fn default() -> Self {
        Self {
            name: Some(String::from("")),
            version: Some(String::from("1.0.0")),
            main: Some(String::from("index.js")),
            author: Some(String::from("")),
            license: Some(String::from("MIT")),
            dependencies: None,
            dev_dependencies: None,
        }
    }
}

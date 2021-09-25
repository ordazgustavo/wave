use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub version: Option<String>,
    pub main: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
}

impl Package {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for Package {
    fn default() -> Self {
        Self {
            name: Default::default(),
            version: Some(String::from("1.0.0")),
            main: Some(String::from("index.js")),
            author: Some(String::from("")),
            license: Some(String::from("MIT")),
        }
    }
}

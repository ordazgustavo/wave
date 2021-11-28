use std::{collections::BTreeMap, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WaveLockfile {
    pub lockfile_version: u32,
    pub packages: ResolvedPackages,
}

pub type ResolvedPackages = BTreeMap<String, ResolvedPackage>;

impl WaveLockfile {
    pub fn new(packages: ResolvedPackages) -> Self {
        Self {
            lockfile_version: 1,
            packages,
        }
    }

    pub fn from_json(pkg: &str) -> Result<Self> {
        serde_json::from_str(pkg).context("Reading lockfile")
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Generating lockfile")
    }

    pub fn location<'a>() -> &'a Path {
        Path::new("wave.lock")
    }

    pub fn is_defined() -> bool {
        let lockfile_path = WaveLockfile::location();
        lockfile_path.exists()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResolvedPackage {
    pub version: String,
    pub resolved: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
}

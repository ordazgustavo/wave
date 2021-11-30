use std::{collections::BTreeMap, fs, path::Path};

use anyhow::{Context, Result};
use node_semver::{Range, Version};
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
        let lockfile_path = Self::location();
        lockfile_path.exists()
    }

    pub fn read() -> Option<Self> {
        if Self::is_defined() {
            match fs::read_to_string(Self::location()).ok() {
                Some(lockfile) => Self::from_json(&lockfile).ok(),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn get_package(&self, name: &str, version: &str) -> Option<ResolvedPackage> {
        self.packages.get(name).and_then(|locked_package| {
            locked_package
                .version
                .parse::<Range>()
                .ok()
                .and_then(|range| {
                    version.parse::<Version>().ok().and_then(|version| {
                        range.satisfies(&version).then(|| locked_package.clone())
                    })
                })
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResolvedPackage {
    pub version: String,
    pub resolved: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<BTreeMap<String, String>>,
}

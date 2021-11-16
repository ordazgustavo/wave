use std::env;
use std::fs::File;
use std::io::prelude::*;
// use std::os::unix;
use std::path::Path;

use anyhow::{Context, Result};

// A simple implementation of `% cat path`
pub fn cat(path: &Path) -> Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s).context(format!(
        "{}: {}",
        "Failed to read file",
        path.as_os_str().to_str().unwrap_or("unknown")
    ))?;
    Ok(s)
}

pub fn cwd() -> Result<String> {
    let path = env::current_dir().context("Couldn't get cwd")?;
    let dir = path.file_name().context("Couldn't get cwd")?;
    let dir = dir.to_os_string();
    let dir = dir.to_str().context("Couldn't get cwd")?;
    Ok(String::from(dir))
}

// A simple implementation of `% echo s > path`
pub fn echo(s: &str, path: &Path) -> Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes()).context(format!(
        "{}: {}",
        "Couldn't write file",
        path.as_os_str().to_str().unwrap_or("unknown")
    ))?;
    Ok(())
}

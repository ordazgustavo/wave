use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
// use std::os::unix;
use std::path::Path;

// A simple implementation of `% cat path`
pub fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn cwd() -> Option<String> {
    let path = env::current_dir();
    match path {
        Ok(path) => {
            let dir = path.file_name();
            if let Some(dir) = dir {
                let dir = dir.to_os_string().into_string();
                match dir {
                    Ok(name) => Some(name),
                    Err(_) => None,
                }
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

// A simple implementation of `% echo s > path`
pub fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

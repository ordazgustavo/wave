use std::fs;
use std::path::Path;

use anyhow::Result;
use bytes::{buf::Reader, Buf, Bytes};
use flate2::read::GzDecoder;
use tar::Archive;

use crate::WaveContext;

pub fn decode_tarball(bytes: Bytes) -> Archive<GzDecoder<Reader<Bytes>>> {
    let tar = GzDecoder::new(bytes.reader());
    Archive::new(tar)
}

pub async fn get_package_tarball(ctx: &WaveContext, url: &str) -> Result<Bytes> {
    Ok(ctx.client.get(url).send().await?.bytes().await?)
}

pub fn save_package_in_node_modules<T>(name: &str, archive: &mut Archive<T>) -> Result<()>
where
    T: std::io::Read,
{
    // The folder name of the unpacked tarball
    let prefix = "package";
    let node_modules = format!("node_modules/{}", name);
    let dest = Path::new(&node_modules);
    fs::create_dir_all(dest)?;

    for mut entry in archive.entries()?.filter_map(|e| e.ok()).into_iter() {
        let path = entry.path()?.strip_prefix(prefix)?.to_owned();
        let path = dest.join(path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        entry.unpack(path)?;
    }

    Ok(())
}

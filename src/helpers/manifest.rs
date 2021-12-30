use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use super::save::get_saves_path;
use crate::consts::SAVES_MANIFEST_NAME;
use crate::models::manifest::Manifest;

#[inline]
pub fn get_manifest_path() -> Result<PathBuf> {
    let path = get_saves_path()?.join(SAVES_MANIFEST_NAME);
    Ok(path)
}

#[inline]
pub fn read_manifest() -> Result<Manifest> {
    let path = get_manifest_path()?;

    let manifest_str = fs::read_to_string(path)?;
    let manifest = toml::from_str(&manifest_str)?;

    Ok(manifest)
}

#[inline]
pub fn write_manifest(manifest: &Manifest) -> Result<()> {
    let saves_path = get_saves_path()?;

    if !saves_path.exists() {
        fs::create_dir_all(saves_path)?;
    }

    let manifest_path = get_manifest_path()?;
    let manifest_str = toml::to_string(manifest)?;

    fs::write(manifest_path, manifest_str)?;

    Ok(())
}

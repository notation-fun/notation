use std::path::PathBuf;

use super::BevyUtil;

impl BevyUtil {
    pub fn check_assets_path(root: PathBuf) -> Option<PathBuf> {
        let mut path = root.clone();
        path.push("assets");
        if !path.exists() && path.is_dir() {
            println!(
                "BevyUtil check_assets_path() not exist: {:?} -> {:?}",
                root, path
            );
        } else if !path.is_dir() {
            println!(
                "BevyUtil check_assets_path() is not dir: {:?} -> {:?}",
                root, path
            );
        } else {
            return Some(path);
        }
        None
    }
    pub fn get_assets_path() -> Option<PathBuf> {
        let mut path = None;
        if let Ok(root) = std::env::current_exe() {
            if let Some(root) = root.parent() {
                path = Self::check_assets_path(root.to_path_buf());
            }
        }
        if path.is_none() {
            if let Ok(root) = std::env::current_dir() {
                path = Self::check_assets_path(root.to_path_buf());
            }
        }
        path
    }
    fn _get_asset_path(root: PathBuf, name: &str, extension: &str) -> Option<PathBuf> {
        let mut path = root.clone();
        path.push(name);
        path.set_extension(extension);
        if path.exists() {
            Some(path)
        } else {
            println!(
                "BevyUtil check_asset_path() not exist: {:?} {}.{} -> {:?}",
                root, name, extension, path
            );
            None
        }
    }
    pub fn get_asset_path(name: &str, extension: &str) -> Option<PathBuf> {
        match Self::get_assets_path() {
            Some(root) => Self::_get_asset_path(root, name, extension),
            None => None,
        }
    }
}

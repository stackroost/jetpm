use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Returns the global package path: ~/.neonpack/lib/<pkg>/<version>
pub fn get_global_path(pkg: &str, version: &str) -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".neonpack/lib")
        .join(pkg)
        .join(version)
}

/// Returns the internal project path: ./neonpack_modules/<pkg>/<version>
pub fn get_internal_path(pkg: &str, version: &str) -> PathBuf {
    env::current_dir()
        .expect("Failed to get current directory")
        .join("neonpack_modules")
        .join(pkg)
        .join(version)
}

/// Returns true if inside a project (package.json exists)
pub fn is_inside_project() -> bool {
    env::current_dir()
        .expect("Failed to get current directory")
        .join("package.json")
        .exists()
}

/// Get the latest installed version of a package from global store
pub fn get_latest_installed_version(pkg: &str) -> Option<String> {
    let base_dir = dirs::home_dir()?.join(".neonpack/lib").join(pkg);
    if !base_dir.exists() {
        return None;
    }

    let mut versions: Vec<String> = fs::read_dir(&base_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    versions.sort();
    versions.pop()
}

/// Get pinned version from neonpack-config.toml
pub fn get_version_from_config(pkg: &str) -> Option<String> {
    use toml_edit::Document;
    let config_path = Path::new("neonpack-config.toml");

    if !config_path.exists() {
        return None;
    }

    let data = fs::read_to_string(config_path).ok()?;
    let doc = data.parse::<Document<String>>().ok()?;
    doc["dependencies"]
        .get(pkg)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

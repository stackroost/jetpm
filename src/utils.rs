use std::path::{PathBuf};
use directories::BaseDirs;
use std::env;

pub fn get_global_path(pkg: &str, version: &str) -> PathBuf {
    BaseDirs::new().unwrap()
        .home_dir()
        .join(".jetpm/lib")
        .join(pkg)
        .join(version)
}

pub fn get_internal_path(pkg: &str, version: &str) -> PathBuf {
    env::current_dir().unwrap()
        .join("jetpm_modules")
        .join(pkg)
        .join(version)
}

pub fn is_inside_project() -> bool {
    env::current_dir().unwrap().join("package.json").exists()
}

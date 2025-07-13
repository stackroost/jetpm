use std::path::PathBuf;
use directories::BaseDirs;

pub fn get_global_package_dir(name: &str) -> PathBuf {
    let base = BaseDirs::new().unwrap();
    base.home_dir().join(".jetpm").join("lib").join(name)
}

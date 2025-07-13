use std::fs;
use std::path::PathBuf;
use crate::utils::get_global_path;

/// Activates (symlinks) the latest installed version of a package into `node_modules`
pub fn use_package(pkg: &str) {
    // Find latest version installed under ~/.jetpm/lib/<pkg>/
    let base_path = get_global_path(pkg, ""); // We'll trim version from here

    // Get list of versions under ~/.jetpm/lib/pkg/
    let version_root = base_path.parent().unwrap();
    let versions = fs::read_dir(version_root).unwrap();

    let mut latest_version: Option<String> = None;

    for entry in versions {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            let name = entry.file_name().into_string().unwrap();
            latest_version = Some(name);
        }
    }

    if latest_version.is_none() {
        println!("No installed versions of '{}' found.", pkg);
        return;
    }

    let version = latest_version.unwrap();
    let real_path = get_global_path(pkg, &version);
    let node_modules = PathBuf::from("node_modules");

    if !node_modules.exists() {
        fs::create_dir_all(&node_modules).unwrap();
    }

    let link_path = node_modules.join(pkg);

    if link_path.exists() {
        fs::remove_file(&link_path).ok();
    }

    std::os::unix::fs::symlink(&real_path, &link_path).unwrap();

    println!("Linked {}@{} → node_modules/{}", pkg, version, pkg);
}

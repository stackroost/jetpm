use std::fs;
use std::path::PathBuf;
use crate::utils::get_global_path;

pub fn uninstall_package(pkg: &str) {
    // Try to remove all versions installed under ~/.jetpm/lib/pkg
    let base_path = get_global_path(pkg, ""); // returns ~/.jetpm/lib/pkg/
    let root_path = base_path.parent().unwrap();

    if !root_path.exists() {
        println!("Package '{}' is not installed.", pkg);
        return;
    }

    match fs::remove_dir_all(root_path) {
        Ok(_) => {
            println!("Uninstalled '{}'", pkg);
        }
        Err(err) => {
            eprintln!("Failed to uninstall '{}': {}", pkg, err);
        }
    }

    // Also remove node_modules symlink if it exists
    let link_path = PathBuf::from("node_modules").join(pkg);
    if link_path.exists() {
        if let Err(e) = fs::remove_file(&link_path) {
            eprintln!("Failed to remove symlink: {}", e);
        } else {
            println!("Removed link from node_modules/{}", pkg);
        }
    }
}

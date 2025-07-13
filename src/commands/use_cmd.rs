use crate::utils::get_global_path;
use std::fs;
use std::path::PathBuf;

pub fn use_package(pkg: &str) {
    let base_dir = dirs::home_dir().unwrap().join(".jetpm/lib").join(pkg);

    if !base_dir.exists() {
        println!("Package '{}' is not installed.", pkg);
        return;
    }

    let mut latest_version: Option<String> = None;
    if let Ok(entries) = fs::read_dir(&base_dir) {
        let mut versions: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect();
        versions.sort();
        latest_version = versions.pop();
    }

    let Some(version) = latest_version else {
        println!("No installed versions of '{}' found.", pkg);
        return;
    };

    let real_path = get_global_path(pkg, &version);
    if !real_path.exists() {
        println!("Package version path not found: {}", real_path.display());
        return;
    }

    let node_modules = PathBuf::from("node_modules");
    if !node_modules.exists() {
        if let Err(e) = fs::create_dir_all(&node_modules) {
            eprintln!("Failed to create node_modules directory: {}", e);
            return;
        }
    }

    let link_path = node_modules.join(pkg);

    if let Ok(meta) = fs::symlink_metadata(&link_path) {
        let file_type = meta.file_type();
        let result = if file_type.is_symlink() || file_type.is_file() {
            fs::remove_file(&link_path)
        } else if file_type.is_dir() {
            fs::remove_dir_all(&link_path)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unknown file type",
            ))
        };

        if let Err(e) = result {
            eprintln!("Failed to remove existing path: {}", e);
            return;
        }
    }

    #[cfg(target_family = "unix")]
    {
        if let Err(e) = std::os::unix::fs::symlink(&real_path, &link_path) {
            eprintln!("Failed to create symlink: {}", e);
            return;
        }
    }

    #[cfg(target_family = "windows")]
    {
        if let Err(e) = std::os::windows::fs::symlink_dir(&real_path, &link_path) {
            eprintln!("Failed to create symlink: {}", e);
            return;
        }
    }

    println!("{}@{} linked to node_modules/{}", pkg, version, pkg);
}

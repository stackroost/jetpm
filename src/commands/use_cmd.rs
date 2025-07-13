use std::fs;

use crate::utils::get_global_package_dir;

pub fn use_package(name: &str) {
    let global_dir = get_global_package_dir(name);
    let current_dir = std::env::current_dir().unwrap();
    let target_link = current_dir.join("node_modules").join(name);

    println!("Linking '{}' to {:?}", name, target_link);

    fs::create_dir_all(target_link.parent().unwrap()).unwrap();

    if target_link.exists() {
        fs::remove_file(&target_link).unwrap();
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(&global_dir, &target_link).unwrap();

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&global_dir, &target_link).unwrap();

    println!("Package '{}' linked to this project.", name);
}

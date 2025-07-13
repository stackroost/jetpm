use std::fs;

use crate::utils::get_global_package_dir;

pub fn install_package(name: &str) {
    let global_dir = get_global_package_dir(name);

    println!("Installing '{}' to {:?}", name, global_dir);

    fs::create_dir_all(&global_dir).unwrap();
    fs::write(global_dir.join("index.js"), "console.log('Hello from global');").unwrap();
    fs::write(global_dir.join("package.json"), format!("{{\"name\": \"{}\"}}", name)).unwrap();

    println!("Package '{}' installed globally.", name);
}

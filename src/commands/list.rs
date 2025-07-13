use std::fs;
use directories::BaseDirs;

pub fn list_packages() {
    let base = BaseDirs::new().unwrap();
    let lib_path = base.home_dir().join(".jetpm/lib");

    if !lib_path.exists() {
        println!("No packages installed.");
        return;
    }

    println!("Installed packages:\n");

    let packages = fs::read_dir(&lib_path).unwrap();

    for pkg_entry in packages {
        let pkg_entry = pkg_entry.unwrap();
        let pkg_name = pkg_entry.file_name().into_string().unwrap();

        let versions_path = pkg_entry.path();
        let mut versions = Vec::new();

        if versions_path.is_dir() {
            for ver_entry in fs::read_dir(versions_path).unwrap() {
                let ver_entry = ver_entry.unwrap();
                let ver_name = ver_entry.file_name().into_string().unwrap();
                versions.push(ver_name);
            }
        }

        println!("- {}: {}", pkg_name, versions.join(", "));
    }
}

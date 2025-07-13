use std::{
    fs,
};

use flate2::read::GzDecoder;
use tar::Archive;
use reqwest::blocking::Client;
use serde_json::Value;
use std::io::Cursor;

use crate::utils::{get_global_path, get_internal_path, is_inside_project};

pub fn install_package(pkg: &str, is_internal: bool) {
    let client = Client::new();
    let url = format!("https://registry.npmjs.org/{}", pkg);

    println!("Fetching metadata for '{}'", pkg);
    let res = client.get(&url).send().unwrap();
    if !res.status().is_success() {
        eprintln!("Package '{}' not found on npm registry.", pkg);
        return;
    }

    let metadata: Value = res.json().unwrap();
    let version = metadata["dist-tags"]["latest"]
        .as_str()
        .expect("Missing latest tag");

    let tarball_url = metadata["versions"][version]["dist"]["tarball"]
        .as_str()
        .expect("Missing tarball URL");

    println!("Downloading tarball for {}@{}", pkg, version);
    let tarball = client.get(tarball_url).send().unwrap().bytes().unwrap();

    // Extract to proper path
    let extract_path = if is_internal && is_inside_project() {
        get_internal_path(pkg, version)
    } else {
        get_global_path(pkg, version)
    };

    if extract_path.exists() {
        println!("{}@{} is already installed.", pkg, version);
        return;
    }

    println!("Installing to {}", extract_path.display());
    let tar = GzDecoder::new(Cursor::new(tarball));
    let mut archive = Archive::new(tar);

    archive.unpack(&extract_path).unwrap();

    // Move extracted "package" folder contents one level up
    let package_folder = extract_path.join("package");
    if package_folder.exists() {
        for entry in fs::read_dir(package_folder).unwrap() {
            let entry = entry.unwrap();
            let dest = extract_path.join(entry.file_name());
            fs::rename(entry.path(), dest).unwrap();
        }
        fs::remove_dir_all(extract_path.join("package")).unwrap();
    }

    println!("Installed {}@{} successfully", pkg, version);
}

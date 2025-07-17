use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use clap::Args;
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use serde_json::Value;
use tar::Archive;

#[derive(Args)]
pub struct InstallArgs {
    /// Optional package name (e.g., lodash). If omitted, install from package.json
    pub package: Option<String>,
}

pub fn run(args: InstallArgs) {
    if let Some(pkg) = args.package {
        install_package(&pkg);
    } else {
        install_from_package_json();
    }
}

fn install_from_package_json() {
    let content = fs::read_to_string("package.json")
        .expect("package.json not found in current directory");

    let json: Value = serde_json::from_str(&content).expect("Invalid JSON in package.json");

    let deps = json.get("dependencies").and_then(|d| d.as_object());

    if deps.is_none() {
        println!("No dependencies found in package.json");
        return;
    }

    for (pkg, _) in deps.unwrap() {
        install_package(pkg);
    }

    println!("All dependencies installed.");
}

fn install_package(pkg: &str) {
    println!("Installing: {}", pkg);

    let url = format!("https://registry.npmjs.org/{}", pkg);
    let response = get(&url).expect("Failed to fetch package metadata");
    let meta: Value = response.json().expect("Invalid npm response");

    let latest_version = meta
        .get("dist-tags")
        .and_then(|tags| tags.get("latest"))
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| {
            eprintln!("Package '{}' not found on npm", pkg);
            std::process::exit(1);
        });

    let tarball_url = meta
        .get("versions")
        .and_then(|v| v.get(latest_version))
        .and_then(|ver| ver.get("dist"))
        .and_then(|d| d.get("tarball"))
        .and_then(|t| t.as_str())
        .unwrap_or_else(|| {
            eprintln!("Tarball URL not found for '{}@{}'", pkg, latest_version);
            std::process::exit(1);
        });

    let resp = get(tarball_url).expect("Failed to download tarball");
    let bytes = resp.bytes().expect("Failed to read tarball");

    let global_path = get_global_path(pkg, latest_version);
    if global_path.exists() {
        println!("Already installed: {}/{}", pkg, latest_version);
    } else {
        extract_tarball(&bytes, &global_path);
        println!("Installed to {}", global_path.display());
    }
}

fn get_global_path(pkg: &str, version: &str) -> PathBuf {
    dirs::home_dir()
        .expect("No home directory")
        .join(".neonpack/lib")
        .join(pkg)
        .join(version)
}

fn extract_tarball(data: &bytes::Bytes, target: &Path) {
    let tar = GzDecoder::new(Cursor::new(data));
    let mut archive = Archive::new(tar);

    archive
        .entries()
        .expect("Failed to read tar entries")
        .filter_map(Result::ok)
        .for_each(|mut entry| {
            let path = entry.path().expect("Invalid path").into_owned();
            let rel_path = path.strip_prefix("package").unwrap_or(&path);
            let final_path = target.join(rel_path);

            if let Some(parent) = final_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create directory");
            }

            entry.unpack(&final_path).expect("Failed to extract file");
        });
}

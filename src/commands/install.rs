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
    pub package: String,
}

pub fn run(args: InstallArgs) {
    let pkg = args.package;
    println!("Installing: {}", pkg);

    let url = format!("https://registry.npmjs.org/{}", pkg);
    let response = get(&url).expect("Failed to fetch package metadata");
    let meta: Value = response.json().expect("Invalid npm response");

    let latest_version = match meta
        .get("dist-tags")
        .and_then(|tags| tags.get("latest"))
        .and_then(|v| v.as_str())
    {
        Some(v) => v,
        None => {
            eprintln!("Package '{}' not found on npm", pkg);
            std::process::exit(1);
        }
    };
    let tarball_url = match meta
        .get("versions")
        .and_then(|v| v.get(latest_version))
        .and_then(|ver| ver.get("dist"))
        .and_then(|d| d.get("tarball"))
        .and_then(|t| t.as_str())
    {
        Some(url) => url,
        None => {
            eprintln!("Tarball URL not found for '{}@{}'", pkg, latest_version);
            std::process::exit(1);
        }
    };

    let resp = get(tarball_url).expect("Failed to download tarball");
    let bytes = resp.bytes().expect("Failed to read tarball");

    let global_path = get_global_path(&pkg, latest_version);
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

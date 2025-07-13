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

    // 1. Get latest version info from npm
    let url = format!("https://registry.npmjs.org/{}", pkg);
    let response = get(&url).expect("Failed to fetch package metadata");
    let meta: Value = response.json().expect("Invalid npm response");

    let latest_version = meta["dist-tags"]["latest"]
        .as_str()
        .expect("Failed to get latest version");

    let tarball_url = meta["versions"][latest_version]["dist"]["tarball"]
        .as_str()
        .expect("Failed to get tarball URL");

    // 2. Download and extract
    let resp = get(tarball_url).expect("Failed to download tarball");
    let bytes = resp.bytes().expect("Failed to read tarball");

    let global_path = get_global_path(&pkg, latest_version);
    if global_path.exists() {
        println!("Already installed: {}/{}", pkg, latest_version);
    } else {
        extract_tarball(&bytes, &global_path);
        println!("Installed to {}", global_path.display());
    }

    // 3. Update config & lock
    update_config_toml(&pkg, latest_version);
    update_lock_json(&pkg, latest_version, &global_path);
}

// ---------------------- Helpers ----------------------

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

    // Extract into a temp dir first (optional)
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

fn update_config_toml(pkg: &str, version: &str) {
    use toml_edit::{value, DocumentMut, Item, Table};

    let path = Path::new("neonpack-config.toml");

    let mut doc = if path.exists() {
        let content = fs::read_to_string(path).expect("Failed to read config");
        content.parse::<DocumentMut>().expect("Invalid TOML")
    } else {
        // Create a new document with package and dependencies sections
        let default_config = String::from(
            r#"[package]
name = "my-app"
version = "0.1.0"

[dependencies]
"#,
        );
        default_config
            .parse::<DocumentMut>()
            .expect("Failed to create new config")
    };

    // Ensure dependencies table exists and insert package
    let dependencies = doc
        .entry("dependencies")
        .or_insert(Item::Table(Table::new()));
    if let Item::Table(table) = dependencies {
        table.insert(pkg, value(version));
    }

    // Save updated config
    fs::write(path, doc.to_string()).expect("Failed to write neonpack-config.toml");
    println!("Updated neonpack-config.toml");
}

fn update_lock_json(pkg: &str, version: &str, path: &Path) {
    use serde_json::{json, Map};

    let lock_path = Path::new("neonpack-lock.json");

    let mut lock: Map<String, Value> = if lock_path.exists() {
        let data = fs::read_to_string(lock_path).expect("Failed to read lock");
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Map::new()
    };

    lock.insert(
        pkg.to_string(),
        json!({
            "version": version,
            "path": path.to_str().unwrap()
        }),
    );

    let output = serde_json::to_string_pretty(&lock).expect("Failed to write lock JSON");
    fs::write(lock_path, output).expect("Failed to save neonpack-lock.json");
    println!("Updated neonpack-lock.json");
}

use std::fs;
use std::path::Path;
use toml_edit::{value};
use toml_edit::Item;
use toml_edit::DocumentMut;
use toml_edit::Table;

/// Pins a globally installed package (from lockfile) to the project config
pub fn use_package(pkg: &str) {
    let config_path = Path::new("neonpack-config.toml");
    let lock_path = Path::new("neonpack-lock.json");

    // Load lockfile
    let lock_data = fs::read_to_string(lock_path)
        .expect("neonpack-lock.json not found. Please install the package globally first.");
    let lock_json: serde_json::Value = serde_json::from_str(&lock_data)
        .expect("Failed to parse neonpack-lock.json");

    let pkg_info = lock_json.get(pkg)
        .expect("Package not found in neonpack-lock.json. Please run `neonpack install <pkg>` first.");

    let version = pkg_info.get("version")
        .and_then(|v| v.as_str())
        .expect("Missing version in lockfile");

    // Load config or create new
    let mut doc = if config_path.exists() {
        let content = fs::read_to_string(config_path).expect("Failed to read neonpack-config.toml");
        content.parse::<DocumentMut>().expect("Invalid TOML in neonpack-config.toml")
    } else {
        // Create a new document with package and dependencies sections
        let default_config = String::from(
            r#"[package]
name = "my-app"
version = "0.1.0"

[dependencies]
"#,
        );
        default_config.parse::<DocumentMut>().expect("Failed to create new config")
    };

    // Ensure dependencies table exists and insert package
    let dependencies = doc.entry("dependencies").or_insert(Item::Table(Table::new()));
    if let Item::Table(table) = dependencies {
        table.insert(pkg, value(version));
    }

    // Save updated config
    fs::write(config_path, doc.to_string()).expect("Failed to write neonpack-config.toml");
    println!("Pinned {}@{} to neonpack-config.toml", pkg, version);
}

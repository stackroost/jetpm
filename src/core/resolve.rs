use std::fs;
use std::path::{Path, PathBuf};
use serde_json::Value;
use toml_edit::DocumentMut;

pub fn resolve_global_import(pkg: &str, subpath: Option<&str>) -> Option<PathBuf> {
    let config_path = Path::new("neonpack-config.toml");
    let lock_path = Path::new("neonpack-lock.json");

    // Load config and lock files
    let config_data = fs::read_to_string(config_path)
        .map_err(|e| eprintln!("Failed to read neonpack-config.toml: {}", e))
        .ok()?;
    let lock_data = fs::read_to_string(lock_path)
        .map_err(|e| eprintln!("Failed to read neonpack-lock.json: {}", e))
        .ok()?;

    // Parse config and lock files
    let config_doc = config_data
        .parse::<DocumentMut>()
        .map_err(|e| eprintln!("Invalid TOML in neonpack-config.toml: {}", e))
        .ok()?;
    let lock_json: Value = serde_json::from_str(&lock_data)
        .map_err(|e| eprintln!("Invalid JSON in neonpack-lock.json: {}", e))
        .ok()?;

    // Get version from dependencies table
    let _version = config_doc
        .get("dependencies")
        .and_then(|deps| deps.as_table())
        .and_then(|table| table.get(pkg))
        .and_then(|item| item.as_str())
        .ok_or_else(|| eprintln!("Package '{}' not found in neonpack-config.toml dependencies", pkg))
        .ok()?;

    // Get package path from lockfile
    let pkg_info = lock_json
        .get(pkg)
        .ok_or_else(|| eprintln!("Package '{}' not found in neonpack-lock.json", pkg))
        .ok()?;
    let pkg_path = PathBuf::from(
        pkg_info
            .get("path")
            .and_then(|p| p.as_str())
            .ok_or_else(|| eprintln!("Missing 'path' for '{}' in neonpack-lock.json", pkg))
            .ok()?,
    );

    // Load and parse package.json
    let package_json_path = pkg_path.join("package.json");
    let package_data = fs::read_to_string(&package_json_path)
        .map_err(|e| eprintln!("Failed to read package.json at {}: {}", package_json_path.display(), e))
        .ok()?;
    let package_json: Value = serde_json::from_str(&package_data)
        .map_err(|e| eprintln!("Invalid JSON in package.json at {}: {}", package_json_path.display(), e))
        .ok()?;

    // Handle subpath (if provided, for future use)
    if let Some(sub) = subpath {
        let subpath_file = pkg_path.join(sub);
        if subpath_file.exists() {
            return Some(subpath_file);
        }
    }

    // Resolve module path based on package.json fields
    if let Some(exports) = package_json.get("exports") {
        if exports.is_object() {
            if let Some(import_val) = exports.get("import") {
                if let Some(entry) = import_val.as_str() {
                    return Some(pkg_path.join(entry));
                }
            }
        } else if exports.is_string() {
            return Some(pkg_path.join(exports.as_str().unwrap()));
        }
    }

    if let Some(module_field) = package_json.get("module") {
        if let Some(module_path) = module_field.as_str() {
            return Some(pkg_path.join(module_path));
        }
    }

    if let Some(main_field) = package_json.get("main") {
        if let Some(main_path) = main_field.as_str() {
            return Some(pkg_path.join(main_path));
        }
    }

    // Default to index.js
    Some(pkg_path.join("index.js"))
}
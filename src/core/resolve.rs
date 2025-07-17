use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub fn resolve_global_import(pkg: &str, subpath: Option<&str>) -> Option<PathBuf> {
    if !is_package_declared(pkg) {
        eprintln!("Package '{}' not declared in package.json", pkg);
        std::process::exit(1);
    }

    let version = get_pinned_version_from_package_json(pkg)
        .or_else(|| get_version_from_config(pkg))
        .or_else(|| get_latest_installed_version(pkg))?;

    let base = dirs::home_dir()?
        .join(".neonpack/lib")
        .join(pkg)
        .join(&version);

    let package_json_path = base.join("package.json");
    let package_data = fs::read_to_string(&package_json_path).ok()?;
    let package_json: Value = serde_json::from_str(&package_data).ok()?;

    if let Some(sub) = subpath {
        let subpath_file = base.join(sub);
        if subpath_file.exists() {
            return Some(subpath_file);
        }
    }

    if let Some(exports) = package_json.get("exports") {
        if exports.is_object() {
            if let Some(import_val) = exports.get("import") {
                if let Some(entry) = import_val.as_str() {
                    return Some(base.join(entry));
                }
            }
        } else if exports.is_string() {
            return Some(base.join(exports.as_str().unwrap()));
        }
    }

    if let Some(module_path) = package_json.get("module").and_then(|m| m.as_str()) {
        return Some(base.join(module_path));
    }

    if let Some(main_path) = package_json.get("main").and_then(|m| m.as_str()) {
        return Some(base.join(main_path));
    }

    Some(base.join("index.js"))
}

fn get_pinned_version_from_package_json(pkg: &str) -> Option<String> {
    let path = Path::new("package.json");
    let content = fs::read_to_string(path).ok()?;
    let json: Value = serde_json::from_str(&content).ok()?;
    json.get("dependencies")?
        .get(pkg)?
        .as_str()
        .map(|s| s.to_string())
}

pub fn get_latest_installed_version(pkg: &str) -> Option<String> {
    let path = Path::new("jetpm-lock.json");
    let content = fs::read_to_string(path).ok()?;
    let json: Value = serde_json::from_str(&content).ok()?;
    let deps = json.get("dependencies")?;
    deps.get(pkg)?.get("version")?.as_str().map(String::from)
}

pub fn is_package_declared(pkg: &str) -> bool {
    let path = Path::new("package.json");
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let json: Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(_) => return false,
    };

    let deps = json.get("dependencies").and_then(|d| d.as_object());
    let dev_deps = json.get("devDependencies").and_then(|d| d.as_object());

    deps.map_or(false, |d| d.contains_key(pkg)) || dev_deps.map_or(false, |d| d.contains_key(pkg))
}

pub fn get_version_from_config(pkg: &str) -> Option<String> {
    let path = Path::new("package.json");
    let content = fs::read_to_string(path).ok()?;
    let json: Value = serde_json::from_str(&content).ok()?;

    json.get("dependencies")
        .and_then(|deps| deps.get(pkg))
        .and_then(|v| v.as_str())
        .map(String::from)
        .or_else(|| {
            json.get("devDependencies")
                .and_then(|dev| dev.get(pkg))
                .and_then(|v| v.as_str())
                .map(String::from)
        })
}

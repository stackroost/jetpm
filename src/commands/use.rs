use serde_json::{Map, Value};
use std::fs;
use std::path::{Path};

/// `neonpack use <pkg>`: Activates package and updates package.json if missing
pub fn use_package(pkg: &str) {
    let pkg_path = Path::new("package.json");

    if !pkg_path.exists() {
        eprintln!("package.json not found");
        std::process::exit(1);
    }

    let content = fs::read_to_string(pkg_path).expect("Failed to read package.json");
    let mut json: Value = serde_json::from_str(&content).expect("Invalid JSON in package.json");

    // Try to get version from dependencies
    let version = json
        .get("dependencies")
        .and_then(|deps| deps.get(pkg))
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
        .or_else(|| get_latest_installed_version(pkg)); // fallback to global

    let version = match version {
        Some(v) => v,
        None => {
            eprintln!(
                "Package '{}' not found locally or globally. Try `neonpack install {}`.",
                pkg, pkg
            );
            std::process::exit(1);
        }
    };

    let global_path = dirs::home_dir()
        .expect("No home directory found")
        .join(".neonpack/lib")
        .join(pkg)
        .join(&version);

    if !global_path.exists() {
        eprintln!(
            "Package '{}'@{} not found in global storage. Try `neonpack install -g {}`.",
            pkg, version, pkg
        );
        std::process::exit(1);
    }

    // If it wasn't in package.json, add it
    if !json.get("dependencies").map_or(false, |v| v.is_object()) {
        json["dependencies"] = Value::Object(Map::new());
    }

    let deps = json["dependencies"]
        .as_object_mut()
        .expect("dependencies must be an object");

    if !deps.contains_key(pkg) {
        deps.insert(pkg.to_string(), Value::String(version.clone()));
        fs::write(pkg_path, serde_json::to_string_pretty(&json).unwrap())
            .expect("Failed to write updated package.json");
        println!("✓ Added '{}'@{} to package.json", pkg, version);
    }

    println!(
        "✓ Activated {}@{} from {}",
        pkg,
        version,
        global_path.display()
    );
}

/// Get latest installed version from ~/.neonpack/lib/<pkg>
fn get_latest_installed_version(pkg: &str) -> Option<String> {
    let base = dirs::home_dir()?.join(".neonpack/lib").join(pkg);
    if !base.exists() {
        return None;
    }

    let mut versions: Vec<String> = fs::read_dir(&base)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    versions.sort();
    versions.pop()
}

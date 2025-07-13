use std::fs;
use std::path::{Path};

pub fn run() {
    let config_path = Path::new("neonpack-config.toml");
    let lock_path = Path::new("neonpack-lock.json");

    if !config_path.exists() {
        fs::write(config_path, "[dependencies]\n").expect("Failed to write neonpack-config.toml");
        println!("Created neonpack-config.toml");
    } else {
        println!("neonpack-config.toml already exists");
    }

    if !lock_path.exists() {
        fs::write(lock_path, "{}").expect("Failed to write neonpack-lock.json");
        println!("Created neonpack-lock.json");
    } else {
        println!("neonpack-lock.json already exists");
    }

    if is_project_present() {
        println!("JavaScript project already detected. Skipping default template.");
        return;
    }

    let src_dir = Path::new("src");
    let index_file = src_dir.join("index.js");

    fs::create_dir_all(src_dir).expect("Failed to create src/ directory");
    fs::write(index_file, "console.log('Hello from neonpack');\n")
        .expect("Failed to write src/index.js");
    println!("Created src/index.js");
}

fn is_project_present() -> bool {
    has_js_or_ts_files(Path::new("."), 2)
}

fn has_js_or_ts_files(dir: &Path, depth: usize) -> bool {
    if depth == 0 || !dir.is_dir() {
        return false;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return false,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "js" | "ts" | "jsx" | "tsx" | "mjs" | "cjs") {
                    return true;
                }
            }
        } else if path.is_dir() {
            if has_js_or_ts_files(&path, depth - 1) {
                return true;
            }
        }
    }

    false
}

use crate::core::rewrite::rewrite_imports;
use clap::Args;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Args)]
pub struct RunArgs {
    pub script: String,
}

pub fn run(args: RunArgs) {
    let pkg_path = find_package_json().unwrap_or_else(|| {
        eprintln!("package.json not found");
        std::process::exit(1);
    });

    let content = fs::read_to_string(&pkg_path).expect("Failed to read package.json");
    let json: Value = serde_json::from_str(&content).expect("Invalid JSON in package.json");

    let cmd = json
        .get("scripts")
        .and_then(|s| s.get(&args.script))
        .and_then(|s| s.as_str())
        .unwrap_or_else(|| {
            eprintln!("script '{}' not found in package.json", args.script);
            std::process::exit(1);
        });

    if let Some(entry_file) = extract_entry_file(cmd) {
        let input = Path::new(&entry_file);
        let output = Path::new(".neonpack-build").join(input.file_name().unwrap());

        if !input.exists() {
            eprintln!("File '{}' not found", input.display());
            std::process::exit(1);
        }

        if let Err(e) = rewrite_imports(&input, &output) {
            eprintln!("Failed to rewrite imports: {}", e);
            std::process::exit(1);
        }

        println!("Running script '{}': node {}", args.script, output.display());

        let status = Command::new("node")
            .arg(output)
            .current_dir(pkg_path.parent().unwrap_or_else(|| Path::new(".")))
            .status()
            .expect("Failed to run script");

        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
    } else {
        println!("Running script '{}': {}", args.script, cmd);

        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(pkg_path.parent().unwrap_or_else(|| Path::new(".")))
            .status()
            .expect("Failed to run script");

        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
    }
}

fn find_package_json() -> Option<PathBuf> {
    let mut dir = env::current_dir().ok()?;
    loop {
        let pkg = dir.join("package.json");
        if pkg.is_file() {
            return Some(pkg);
        }
        if !dir.pop() {
            break;
        }
    }
    None
}

fn extract_entry_file(cmd: &str) -> Option<String> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() == 2 && parts[0] == "node" {
        Some(parts[1].to_string())
    } else {
        None
    }
}

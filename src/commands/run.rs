use crate::core::rewrite::rewrite_imports;
use clap::Args;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Args)]
pub struct RunArgs {
    pub script: String,
}

pub fn run(args: RunArgs) {
    if args.script == "dev" {
        run_dev();
    } else {
        run_script(&args.script);
    }
}

fn run_dev() {
    let input = Path::new("src/index.js");
    let output = Path::new(".neonpack-build/index.js");

    if !input.exists() {
        eprintln!("src/index.js not found");
        std::process::exit(1);
    }

    if let Err(e) = rewrite_imports(input, output) {
        eprintln!("Rewrite failed: {}", e);
        std::process::exit(1);
    }

    let status = Command::new("node")
        .arg(output)
        .status()
        .expect("Failed to start Node.js");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn run_script(script: &str) {
    let pkg_path = PathBuf::from("package.json");
    let content = fs::read_to_string(&pkg_path).expect("package.json not found");
    let json: Value = serde_json::from_str(&content).expect("Invalid package.json");

    let cmd = json
        .get("scripts")
        .and_then(|s| s.get(script))
        .and_then(|s| s.as_str())
        .unwrap_or_else(|| {
            eprintln!("Script '{}' not found", script);
            std::process::exit(1);
        });

    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .expect("Failed to run script");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

use std::fs;
use std::path::PathBuf;
use clap::Args;


use crate::core::resolve::{get_latest_installed_version, get_version_from_config};


#[derive(Args)]
pub struct ListArgs {
    #[arg(long)]
    pub used: bool,

    #[arg(long)]
    pub all: bool,
}

pub fn run(args: ListArgs) {
    let show_used = args.used;
    // let show_all = args.all;

    let base = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".neonpack/lib");

    if !base.exists() {
        println!("No global packages installed.");
        return;
    }

    println!("Global packages installed:\n");

    let entries = match fs::read_dir(&base) {
        Ok(entries) => entries,
        Err(_) => {
            println!("Unable to read package directory.");
            return;
        }
    };

    for entry in entries.flatten() {
        let pkg_name = entry.file_name().to_string_lossy().to_string();
        let pkg_path = entry.path();
        let versions = get_versions(&pkg_path);

        for version in versions {
            let in_use = is_version_used(&pkg_name, &version);

            if show_used && !in_use {
                continue;
            }

            let marker = if in_use { "✓ in use" } else { "" };

            println!("  {:20} {:10} {}", pkg_name, version, marker);
        }
    }
}

fn get_versions(pkg_path: &PathBuf) -> Vec<String> {
    fs::read_dir(pkg_path)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let name = entry.ok()?.file_name().into_string().ok()?;
            Some(name)
        })
        .collect()
}

fn is_version_used(pkg: &str, version: &str) -> bool {
    get_version_from_config(pkg).as_deref() == Some(version)
        || get_latest_installed_version(pkg).as_deref() == Some(version)
}

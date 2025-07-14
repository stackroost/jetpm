use std::fs;
use std::path::Path;
use clap::Args;
use serde_json::Value;

#[derive(Args)]
pub struct RemoveArgs {
    pub name: String,
    #[arg(short, long)]
    pub global: bool,
}

pub fn run(args: RemoveArgs) {
    if args.global {
        let global_path = dirs::home_dir()
            .expect("No home directory found")
            .join(".neonpack/lib")
            .join(&args.name);

        if global_path.exists() {
            fs::remove_dir_all(&global_path)
                .expect("Failed to remove global package directory");
            println!("✓ Removed '{}' from global storage", args.name);
        } else {
            println!("⚠ '{}' not found in global storage", args.name);
        }

    } else {
        let pkg_path = Path::new("package.json");
        if !pkg_path.exists() {
            eprintln!("package.json not found");
            std::process::exit(1);
        }

        let content = fs::read_to_string(pkg_path).expect("Failed to read package.json");
        let mut json: Value = serde_json::from_str(&content).expect("Invalid JSON in package.json");
        let mut removed = false;

        for section in ["dependencies", "devDependencies"] {
            if let Some(table) = json.get_mut(section) {
                if let Some(deps) = table.as_object_mut() {
                    if deps.remove(&args.name).is_some() {
                        removed = true;
                    }
                }
            }
        }

        if removed {
            fs::write(pkg_path, serde_json::to_string_pretty(&json).unwrap())
                .expect("Failed to write package.json");
            println!("✓ Removed '{}' from package.json", args.name);
        } else {
            eprintln!("Package '{}' not found in dependencies", args.name);
            std::process::exit(1);
        }
    }
}

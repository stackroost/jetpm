mod commands;
mod utils;
mod welcome;

use clap::{Parser, Subcommand};
use commands::{install::install_package, use_cmd::use_package};
use commands::uninstall::uninstall_package;
use welcome::show_welcome;

/// JetPM - Jet-fast global JavaScript package manager
#[derive(Parser)]
#[command(
    name = "jetpm",
    version,
    about = "Jet-fast global JavaScript package manager"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a package from the npm registry
    Install {
        /// Package name (e.g., react or react@18.2.0)
        name: String,

        /// Install inside local project (jetpm_modules/) instead of globally
        #[arg(short, long)]
        internal: bool,
    },

    /// Use a previously installed global package in the current project
    Use {
        /// Package name (e.g., chalk)
        name: String,
    },
    Uninstall {
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install { name, internal }) => {
            install_package(&name, internal);
        }
        Some(Commands::Use { name }) => {
            use_package(&name);
        }
        Some(Commands::Uninstall { name }) => {
            uninstall_package(&name);
        }
        None => {
            show_welcome();
        }
    }
}

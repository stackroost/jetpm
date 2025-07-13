mod commands;
mod utils;
mod welcome;

use clap::{Parser, Subcommand};
use commands::list::list_packages;
use commands::uninstall::uninstall_package;
use commands::{install::install_package, use_cmd::use_package};
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
    Install {
        name: String,
        #[arg(short, long)]
        internal: bool,
    },
    Use {
        name: String,
    },
    List,
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
        Some(Commands::List) => {
            list_packages();
        }
        None => {
            show_welcome();
        }
    }
}

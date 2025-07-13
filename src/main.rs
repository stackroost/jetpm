mod commands;
mod utils;
mod welcome;

use clap::{Parser, Subcommand};
use commands::{install::install_package, use_cmd::use_package};
use welcome::show_welcome;

#[derive(Parser)]
#[command(name = "jetpm", version, about = "Jet-fast global package manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Install {
        name: String,
    },
    Use {
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Install { name }) => install_package(name),
        Some(Commands::Use { name }) => use_package(name),
        None => show_welcome(),
    }
}

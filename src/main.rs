use clap::{Parser, error::ErrorKind};
mod core;
mod commands;
mod welcome;

#[derive(Parser)]
#[command(
    name = "neonpack",
    disable_help_flag = true,    
    disable_version_flag = true  
)]
struct Cli {
    #[command(subcommand)]
    command: Option<commands::Command>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match Cli::try_parse_from(&args) {
        Ok(cli) => match cli.command {
            Some(cmd) => cmd.execute(),
            None => welcome::show_welcome(),
        },
        Err(e) => {
            if e.kind() == ErrorKind::DisplayHelp {
                welcome::show_welcome();
            } else {
                e.print().expect("Error writing error");
                std::process::exit(1);
            }
        }
    }
}

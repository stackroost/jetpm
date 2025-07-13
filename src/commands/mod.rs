pub mod init;
pub mod install;
pub mod run;
pub mod r#use;


use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    Run(run::RunArgs),
    Init,
    Install(install::InstallArgs),
    Use { package: String },
}

impl Command {
    pub fn execute(self) {
        match self {
            Command::Run(args) => run::run(args),
            Command::Init => init::run(),
            Command::Install(args) => install::run(args),
            Command::Use { package } => r#use::use_package(&package),
        }
    }
}

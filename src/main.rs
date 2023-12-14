use clap::{Parser, Subcommand};
use cmds::PkgCmd;

mod cmds;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile and/or install project's PyPI requirements into virtualenv
    Pkg(PkgCmd),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Pkg(cmd) => {
            cmd.run();
        }
    }
}

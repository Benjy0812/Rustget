mod cli;
mod commands;
mod hash;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { name } => commands::install(name),
        Commands::Remove { name } => commands::uninstall(name),
        Commands::Search { term } => commands::search(term),
        Commands::Sha256 { path } => commands::sha256(path),
        Commands::List => commands::list(),
        Commands::Info => commands::info(),
    }
}

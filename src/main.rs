use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Rustget")]
#[command(version = "0.1")]
#[command(about = "package manager written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install { name: String },
    Remove { name: String },
    Search { term: String },
    List,
    Info,
}

fn main() {
    println!("Welcome to Rustget!");
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { name } => install(name),
        Commands::Remove { name } => uninstall(name),
        Commands::Search { term } => search(term),
        Commands::List => list(),
        Commands::Info => info(),
    }
}

fn install(name: String) {
    println!("Installing {name}");
}

fn uninstall(name: String) {
    println!("Uninstalling {name}");
}

fn list() {
    println!("Listing Isntalled");
}

fn search(term: String) {
    println!("Search {term}");
}

fn info() {
    println!("Info");
}

mod hash;

use clap::{Parser, Subcommand};
use hash::sha256_from_file;
use std::path::PathBuf;

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
    FilePath { path: PathBuf },
    List,
    Info,
}

fn main() {
    println!("Rust Package Manager");
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { name } => install(name),
        Commands::Remove { name } => uninstall(name),
        Commands::Search { term } => search(term),
        Commands::FilePath { path } => sha256(path),
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

fn sha256(path: PathBuf) {
    println!("File Path {}", path.display());
    let sha256 = sha256_from_file(&path).unwrap();
    println!("Sha256:{sha256}");
}

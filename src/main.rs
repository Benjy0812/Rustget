mod database;
mod hash;

use clap::{Parser, Subcommand};
use hash::sha256_from_file;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Install { name } => install(name),
        Commands::Remove { name } => uninstall(name),
        Commands::Search { term } => search(term),
        Commands::Sha256 { path } => sha256(path),
        Commands::List => list(),
        Commands::Info => info(),
    }
}

/// CLI entry point, parsed via clap.
#[derive(Parser)]
#[command(name = "Rustget")]
#[command(version = "0.1")]
#[command(about = "package manager written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Supported subcommands.
#[derive(Subcommand)]
pub enum Commands {
    Install { name: String },
    Remove { name: String },
    Search { term: String },
    Sha256 { path: PathBuf },
    List,
    Info,
}

pub fn install(name: String) {
    println!("Installing {name}");
}

pub fn uninstall(name: String) {
    println!("Uninstalling {name}");
}

pub fn list() {
    println!("Listing Isntalled");
}

pub fn search(term: String) {
    println!("Search {term}");
}

pub fn info() {
    println!("Info");
}

/// Prints the SHA-256 hash of the file at `path`.
pub fn sha256(path: PathBuf) {
    println!("\nFile Path {}\n", path.display());
    let sha256 = sha256_from_file(&path).unwrap(); // panics if file can't be read
    println!("Sha256:{sha256}\n");
}

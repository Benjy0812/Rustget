use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Rustget")]
#[command(version = "0.1")]
#[command(about = "package manager written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install { name: String },
    Remove { name: String },
    Search { term: String },
    FilePath { path: PathBuf },
    List,
    Info,
}

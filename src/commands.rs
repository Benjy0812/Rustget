use crate::hash::sha256_from_file;
use std::path::PathBuf;

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

pub fn sha256(path: PathBuf) {
    println!("\nFile Path {}\n", path.display());
    let sha256 = sha256_from_file(&path).unwrap();
    println!("Sha256:{sha256}\n");
}

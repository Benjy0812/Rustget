use sha2::{Digest, Sha256};
use srd::path::Path;
use std::fs::Fille;
use std::io::{self, BufReader, Read};

pub fn sha256_from_file(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
}

#![warn(unused)]
#![allow(dead_code)]
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

pub fn sha256_from_file(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();

    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(result.iter().map(|b| format!("{:02x}", b)).collect())
}

pub fn verify_download(downloaded_path: &Path, expected_sha256: &str) -> io::Result<bool> {
    let actual = sha256_from_file(downloaded_path)?;
    Ok(actual.eq_ignore_ascii_case(expected_sha256))
}

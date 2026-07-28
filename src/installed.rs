use std::fs;
use std::io;

const INSTALLED_PATH: &str = "installed.json";

pub fn load_installed() -> Vec<String> {
    match fs::read_to_string(INSTALLED_PATH) {
        Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn save_installed(installed: &Vec<String>) -> io::Result<()> {
    let json =
        serde_json::to_string_pretty(installed).expect("Vec<String> should always serialize fine");
    fs::write(INSTALLED_PATH, json)
}

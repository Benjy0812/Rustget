pub use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub category: String,
    pub index: Vec<String>,
    pub packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub homepage: String,
    pub download_url: String,
    pub sha256: String,
}

pub fn load_database(path: &str) -> Result<Database, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;

    let database: Database = serde_json::from_str(&json)?;

    Ok(database)
}

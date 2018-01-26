use std::path::Path;
use std;
use std::fs::File;
use serde_json;
use std::io::Read;
use std::env::home_dir;

type GenError = Box<std::error::Error>;

#[derive(Deserialize, Debug)]
#[derive(Default)]
#[allow(non_snake_case)]
pub struct Config {
    pub serverAddress: Option<String>,
    pub authToken: Option<String>,
}

impl Config {
    pub fn read() -> Result<Self, GenError> {
        match home_dir() {
            Some(mut path) => {
                path.push(".config");
                path.push("acrusto.json");
                Self::read_from_path(&path)
            },
            None => Ok(Config::default()),
        }
    }

    pub fn read_from_path(config_file_path: &Path) -> Result<Self, GenError> {
        if !config_file_path.exists() {
            return Ok(Config::default());
        }

        let mut file = File::open(config_file_path)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        let config: Self = serde_json::from_str(&file_content)?;

        Ok(config)
    }
}
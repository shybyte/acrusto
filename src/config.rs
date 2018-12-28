use std::path::Path;
use std;
use std::fs::File;
use std::io::Read;
use dirs::home_dir;
use serde_derive::Deserialize;

type GenError = Box<std::error::Error>;

#[derive(Deserialize, Debug)]
#[derive(Default)]
pub struct Config {
    pub acrolinx_address: Option<String>,
    pub access_token: Option<String>,
}

impl Config {
    pub fn read() -> Self {
        home_dir().and_then(|mut path| {
            path.push(".config");
            path.push("acrusto.json");
            Self::read_from_path(&path).ok()
        }).unwrap_or_default()
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
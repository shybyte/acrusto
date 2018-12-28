use std::path::Path;
use std;
use std::fs::File;
use std::io::Read;
use dirs::home_dir;
use serde_derive::Deserialize;
use envy;
use log::{info};

type GenError = Box<std::error::Error>;

#[derive(Deserialize, Debug)]
#[derive(Default)]
pub struct Config {
    pub acrolinx_address: Option<String>,
    pub access_token: Option<String>,
}

impl Config {
    pub fn read() -> Self {
        let config_from_env = envy::prefixed("ACROLINX_").from_env::<Config>().unwrap_or_default();
        info!("config_from_env = {:?}", config_from_env);


        let config_from_file = home_dir().and_then(|mut path| {
            path.push(".config");
            path.push("acrusto.json");
            Self::read_from_path(&path).ok()
        }).unwrap_or_default();

        Config {
            acrolinx_address: config_from_env.acrolinx_address.or(config_from_file.acrolinx_address),
            access_token: config_from_env.access_token.or(config_from_file.access_token)
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
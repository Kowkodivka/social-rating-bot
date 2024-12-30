use serde::Deserialize;
use std::fs;

use crate::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Discord {
    pub token: String,
    pub prefix: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub discord: Discord,
}

impl Config {
    pub fn load(file_path: &str) -> Result<Self, Error> {
        let config_contents = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_contents).unwrap();

        Ok(config)
    }
}

use envy::Error as EnvyError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub discord_token: String,
    pub discord_prefix: String,
}

impl Config {
    pub fn load() -> Result<Self, EnvyError> {
        envy::from_env()
    }
}

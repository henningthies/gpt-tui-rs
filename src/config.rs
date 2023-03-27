use serde::{Deserialize, Serialize};
use std::{fs, io::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_token: String
}

impl Config {
    pub fn new(api_token: String) -> Self {
        Self { api_token }
    }

    pub fn read() -> Result<Self, Error> {
        let contents = fs::read_to_string("config/config.json")?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn write(&self) -> Result<(), Error> {
        let contents = serde_json::to_string_pretty(&self)?;
        fs::write("config/config.json", contents)?;
        Ok(())
    }
}

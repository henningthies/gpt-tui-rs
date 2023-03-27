use serde::{Deserialize, Serialize};
use std::{fs, io::{Error, self}};

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

    pub fn get_config() -> Self {
        match Self::read() {
            Ok(config) => {
                if config.api_token.is_empty() {
                    Self::setup()
                } else {
                    config
                }
            },
            Err(_) => Self::setup(),
        }
    }

    fn setup() -> Self {
        println!("Setup config. Please provide your api token:");
        let mut api_token = String::new();
        io::stdin().read_line(&mut api_token).unwrap();
        let config = Self::new(api_token.trim().to_string());
        match config.write() {
            Ok(_) => println!("Config written"),
            Err(_) => println!("Error writing config"),
        }
        config
    }
}

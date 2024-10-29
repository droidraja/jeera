use anyhow::Result;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub(crate) email: String,
    pub(crate) api_token: String,
    pub(crate) host: String,
}

impl Config {
    pub fn write_config(&self) -> Result<()> {
        let config_dir = BaseDirs::new()
            .ok_or_else(|| anyhow::anyhow!("Failed to get base directories"))?
            .home_dir()
            .join(".jeera");
        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("config.json");
        fs::write(config_path, serde_json::to_string(self)?)?;
        Ok(())
    }

    pub fn load_config() -> Result<Config> {
        if let Some(base_dirs) = BaseDirs::new() {
            let config_dir = base_dirs.home_dir();
            let config_path = config_dir.join(".jeera").join("config.json");

            if config_path.exists() {
                let config_str = fs::read_to_string(config_path)?;
                let config: Config = serde_json::from_str(&config_str)?;
                return Ok(config);
            }
        }

        Err(anyhow::anyhow!("No config file found"))
    }
}

impl From<(String, String, String)> for Config {
    fn from((username, password, host): (String, String, String)) -> Self {
        Self {
            email: username,
            api_token: password,
            host,
        }
    }
}

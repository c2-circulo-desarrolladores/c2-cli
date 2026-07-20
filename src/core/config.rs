use std::path::PathBuf;

use anyhow::Context;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub owner: Option<String>,
}

impl Config {
    pub fn config_path() -> anyhow::Result<PathBuf> {
        let dirs = ProjectDirs::from("", "c2", "c2_cli")
            .context("failed to determine config directory")?;
        let config_path = dirs.config_dir().join("config.toml");
        Ok(config_path)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_path()?;

        std::fs::create_dir_all(config_path.parent().unwrap())
            .with_context(|| format!("failed to create {:?}", &config_path.parent()))?;

        let contents = toml::to_string_pretty(&self).context("failed to serialize config")?;

        std::fs::write(&config_path, contents)
            .with_context(|| format!("failed to write {:?}", &config_path))?;

        Ok(())
    }

    pub fn load() -> anyhow::Result<Config> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Config { owner: None });
        }

        let contents =
            std::fs::read_to_string(config_path).context("failed to read config.toml")?;

        let config = toml::from_str(&contents).context("failed to parse config.toml")?;

        Ok(config)
    }
}

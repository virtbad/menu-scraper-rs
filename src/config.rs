use anyhow::{anyhow, Context};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub api_remote: String,
    pub website_remote: String,
}

impl Config {
    /// parse an existing config or create a new one should none exist
    pub fn parse() -> anyhow::Result<Self> {
        let mut cfg: Config = confy::load("menu-scraper", "menu-scraper").with_context(|| "Failed to read config")?;

        // overwrite config values should environment variables be set
        if let Ok(api_remote) = std::env::var("API") {
            info!("Overwriting api_remote with API environment variable");
            cfg.api_remote = api_remote;
        }
        if let Ok(website_remote) = std::env::var("WEBSITE") {
            info!("Overwriting website_remote with WEBSITE environment variable");
            cfg.website_remote = website_remote;
        }

        Ok(cfg)
    }

    /// check whether the config is valid
    pub fn check(&self) -> anyhow::Result<()> {
        if self.api_remote.is_empty() && self.website_remote.is_empty() {
            Err(anyhow!("API remote and website remote have to be set in config"))
        } else if self.api_remote.is_empty() {
            Err(anyhow!("API remote has to be set in config"))
        } else if self.website_remote.is_empty() {
            Err(anyhow!("Website remote has to be set in config"))
        } else {
            info!("Config is OK");
            Ok(())
        }
    }
}

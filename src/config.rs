use std::error::Error;
use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub gui: GuiConfig,
    pub channels: ChannelConfig,
}

#[derive(Deserialize)]
pub struct GuiConfig {
    pub scale: Option<u16>,
}

#[derive(Deserialize)]
pub struct ChannelConfig {
    #[serde(default = "channel_max_default")]
    pub max: f32,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let config = toml::from_slice(&fs::read(path)?)?;
        Ok(config)
    }
}

fn channel_max_default() -> f32 {
    1.0
}

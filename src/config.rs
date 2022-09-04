use std::error::Error;
use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub gui: GuiConfig,
    pub channels: ChannelsConfig,
}

#[derive(Deserialize, Debug)]
pub struct GuiConfig {
    pub scale: Option<u16>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct ChannelsConfig {
    default: ChannelConfig,
    channel1: OptionalChannelConfig,
    channel2: OptionalChannelConfig,
    channel3: OptionalChannelConfig,
    channel4: OptionalChannelConfig,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ChannelConfig {
    pub max: f32,
    pub invert: bool,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct OptionalChannelConfig {
    max: Option<f32>,
    invert: Option<bool>,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        ChannelConfig {
            max: 1.0,
            invert: false,
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let config = toml::from_slice(&fs::read(path)?)?;
        Ok(config)
    }
}

impl ChannelsConfig {
    pub fn channel1(&self) -> ChannelConfig {
        self.channel_config(&self.channel1)
    }
    pub fn channel2(&self) -> ChannelConfig {
        self.channel_config(&self.channel2)
    }
    pub fn channel3(&self) -> ChannelConfig {
        self.channel_config(&self.channel3)
    }
    pub fn channel4(&self) -> ChannelConfig {
        self.channel_config(&self.channel4)
    }

    fn channel_config(&self, config: &OptionalChannelConfig) -> ChannelConfig {
        ChannelConfig {
            max: config.max.unwrap_or(self.default.max),
            invert: config.invert.unwrap_or(self.default.invert),
        }
    }
}

use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use crate::extract::TranslationSource;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub fallback_locale: String,
    pub listen_port: u16,
    #[serde(rename = "source")]
    pub sources: Vec<ConfigSourceWrapper>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fallback_locale: "en_us".to_string(),
            listen_port: 4040,
            sources: vec![
                ConfigSource::Datapacks(PathBuf::from("datapacks/")).into(),
                ConfigSource::Mods(PathBuf::from("mods/")).into(),
            ],
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigSourceWrapper(ConfigSource);

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type", content = "path")]
pub enum ConfigSource {
    Mods(PathBuf),
    Datapacks(PathBuf),
}

impl Into<TranslationSource> for &ConfigSourceWrapper {
    fn into(self) -> TranslationSource {
        match &self.0 {
            ConfigSource::Mods(path) => TranslationSource::ModsDirectory(path.clone()),
            ConfigSource::Datapacks(path) => TranslationSource::DatapacksDirectory(path.clone()),
        }
    }
}

impl Into<ConfigSourceWrapper> for ConfigSource {
    fn into(self) -> ConfigSourceWrapper {
        ConfigSourceWrapper(self)
    }
}

impl Config {
    pub fn sources(&self) -> Vec<TranslationSource> {
        self.sources.iter().map(|source| source.into()).collect::<Vec<_>>()
    }
}

pub(crate) fn load_config() -> Config {
    let path = Path::new("config.toml");
    if path.exists() {
        let mut buf = String::new();
        File::open(path).expect("failed to open config")
            .read_to_string(&mut buf).expect("failed to read config");
        toml::from_str(&buf).expect("failed to parse config")
    } else {
        let config = Config::default();

        let content = toml::to_string(&config).expect("failed to write default config");
        fs::write(path, content).expect("failed to write default config");

        config
    }
}

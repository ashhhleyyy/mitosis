use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use crate::extract::TranslationSource;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub mods_dir: Option<PathBuf>,
    pub datapacks_dir: Option<PathBuf>,

    pub fallback_locale: String,

    pub listen_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mods_dir: Some(PathBuf::from("mods/")),
            datapacks_dir: Some(PathBuf::from("datapacks/")),
            fallback_locale: "en_us".to_string(),
            listen_port: 4040,
        }
    }
}

impl Config {
    pub fn sources(&self) -> Vec<TranslationSource> {
        let mut sources = Vec::new();

        if let Some(mods) = &self.mods_dir {
            sources.push(TranslationSource::ModsDirectory(mods.clone()));
        }

        if let Some(datapacks) = &self.datapacks_dir {
            sources.push(TranslationSource::DatapacksDirectory(datapacks.clone()));
        }

        sources
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

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

use toml;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ThumbnailConfig {
    pub size: u32,
    pub square: bool,
    pub extension: String,
}

#[derive(Deserialize, Clone)]
pub struct DbConfig {
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub gallery_name: String,

    pub storage_path: String,
    pub cache_path: String,

    pub allowed_extensions: HashSet<String>,

    pub small_thumbnail: ThumbnailConfig,
    pub medium_thumbnail: ThumbnailConfig,

    pub db: DbConfig,
}

impl Config {
    pub fn load() -> Self {
        let config_path  = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("gallery.toml");
        let mut config_file = File::open(config_path).expect("Could not find configuration file.");

        let mut content = String::new();
        config_file.read_to_string(&mut content).expect("Something went wrong reading the configuration.");

        let config: Config = toml::from_str(content.as_str()).expect("Could not parse configuration.");
        config
    }
}

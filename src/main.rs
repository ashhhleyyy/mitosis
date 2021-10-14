#[macro_use]
extern crate log;

use std::env;

use crate::config::load_config;
use crate::translations::{load_translations_from, TranslationsHolder};

mod extract;
mod util;
mod config;
mod web;
mod translations;

#[tokio::main]
async fn main() {
    // Set default log level if none set
    if !env::vars().any(|(k, _)| k == "RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    info!("Hello, world!");

    let config = load_config();

    if config.mods_dir.is_none() && config.datapacks_dir.is_none() {
        warn!("No sources are configured for loading translations!");
    }

    let all_translations = load_translations_from(&config.sources()).await
        .expect("failed to load initial sources");

    let translations_holder = TranslationsHolder::new(config.sources(), all_translations);

    web::run(config, translations_holder).await;
}

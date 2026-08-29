#![allow(unused)]

use crate::{ConsoleConfig, GridConfig};
use etcetera::{BaseStrategy, choose_base_strategy};
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.json";
const KEYMAP_FILE_NAME: &str = "keymap.json";
const PROJECT_NAME: &str = "graliffer";

fn config_dir() -> Option<PathBuf> {
    choose_base_strategy()
        .ok()
        .and_then(|strategy| Some(strategy.config_dir().join(PROJECT_NAME)))
}

#[derive(Debug, Default)]
pub struct Config {
    pub console: ConsoleConfig,
    pub grid: GridConfig,
}

impl Config {
    // fn fetch_keymap() -> Option<Keymap> {
    //     Self::config_dir()
    //         .and_then(|config_dir| fs::read_to_string(config_dir.join(Self::KEYMAP_FILE_NAME)).ok())
    //         .and_then(|content| serde_json::from_str(&content).ok())
    // }

    // pub fn fetch() -> Self {
    //     // let config_file = config_dir.to_owned().and_then(|config_dir| {
    //     //     fs::read_to_string(config_dir.join(Self::CONFIG_FILE_NAME)).ok()
    //     // });

    //     let default_keymap = include_str!("../assets/default_keymap.jsonc");

    //     // let default_config = include_str!("../assets/default_config.json");
    // }
}

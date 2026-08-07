#![allow(unused)]

use etcetera::{BaseStrategy, choose_base_strategy};
use std::path::PathBuf;

mod config_content;
pub use config_content::*;

const CONFIG_FILE_NAME: &str = "config.json";
const KEYMAP_FILE_NAME: &str = "keymap.json";
const PROJECT_NAME: &str = "graliffer";

fn config_dir() -> Option<PathBuf> {
    choose_base_strategy()
        .ok()
        .and_then(|strategy| Some(strategy.config_dir().join(PROJECT_NAME)))
}

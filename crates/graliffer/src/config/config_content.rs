#[derive(Debug)]
pub struct ConsoleConfig {
    pub line_history: usize,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        Self { line_history: 1000 }
    }
}

#[derive(Debug, Default)]
pub struct Config {
    pub console: ConsoleConfig,
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

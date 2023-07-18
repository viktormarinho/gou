use pretty_log::log;
use serde::Deserialize;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Debug, Deserialize)]
struct LoadableConfig {
    pub build_command: Option<String>,
    pub main_branch: Option<String>
}

impl Default for LoadableConfig {
    fn default() -> Self {
        LoadableConfig {
            build_command: None,
            main_branch: None
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub build_command: Option<String>,
    pub main_branch: String
}

impl Config {
    pub fn load() -> Self {
        let file_str = std::fs::read_to_string("./gou.toml");

        let config = toml::from_str::<LoadableConfig>(&file_str.unwrap_or_default());

        if let Err(e) = config {
            log::error_exit(
                &format!("[gou] Failed to parse config file: {}", e)
            );
        }

        let config = config.unwrap();

        Config {
            build_command: config.build_command,
            main_branch: config.main_branch.unwrap_or(String::from("main")),
        }
    }
}
use serde::Deserialize;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub build_command: Option<String>,
    pub main_branch: Option<String>
}

impl Config {
    pub fn load() -> Self {
        let file_str = std::fs::read_to_string("./gou.toml").expect("Missing config file");

        let config: Config = toml::from_str(&file_str).expect("Invalid config file");

        return config;
    }
}
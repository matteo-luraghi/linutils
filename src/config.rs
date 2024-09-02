use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Distro {
    pub name: String,
    pub package_manager: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub packages: Vec<String>,
    pub distros: Vec<Distro>,
}

/// Load the config.toml file into a Config object
pub fn load_config(file_path: &str) -> Config {
    let config_content = fs::read_to_string(file_path).expect("Failed to read config file");

    toml::from_str(&config_content).expect("Failed to parse config file")
}

use std::fs;

use chrono::NaiveDate;
use serde::Deserialize;
use tauri::api::path::home_dir;

pub fn get_config() -> Config {
    let mut config_path = home_dir().unwrap();
    config_path.push("mcountdown.toml");

    let file_raw = fs::read_to_string(config_path);

    match file_raw {
        Ok(raw) => toml::from_str(&raw).unwrap(),
        Err(_) => Config {
            name: "master thesis".to_string(),
            target_date: NaiveDate::from_ymd_opt(2050, 12, 5).unwrap(),
        },
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub target_date: chrono::NaiveDate,
}

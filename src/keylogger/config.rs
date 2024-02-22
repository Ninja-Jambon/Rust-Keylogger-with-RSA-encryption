use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub channel_id: String,
    pub token: String,
}

impl Config {
    pub fn new(file_url: &str) -> Config {
        let file_content: String = fs::read_to_string(file_url).expect("Couldn't read file content.");
        let file_content_as_str: &str = file_content.as_str();
        let config: Config = serde_json::from_str(file_content_as_str).expect("Couldn't parse JSON.");

        return config;
    }
}
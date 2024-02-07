mod discord;
use discord::Discord;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    channel_id: String,
    token: String,
}

fn main() {
    let file_content: String = fs::read_to_string("./src/config.json").expect("Couldn't read file content.");
    let file_content_as_str: &str = file_content.as_str();
    let config: Config = serde_json::from_str(file_content_as_str).expect("Couldn't parse JSON.");

    let discord_client: Discord = Discord::new(config.channel_id.as_str(), config.token.as_str());

    discord_client.send_webhook("Hello, Rust !");
}
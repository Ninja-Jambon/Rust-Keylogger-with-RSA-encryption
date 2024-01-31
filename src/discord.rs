use reqwest::blocking::Client;
use std::collections::HashMap;

pub struct Discord {
    url: String
}

impl Discord {
    pub fn new(url: &str) -> Discord {
        Discord {
            url: url.to_string(),
        }
    }

    pub fn send_webhook(&self, content: &str) {
        let client: Client = Client::new();
        let mut map = HashMap::new();

        map.insert("content", content);

        let _res = client.post(&self.url).json(&map).send();
    }
}

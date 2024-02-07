use reqwest::blocking::Client;
use std::collections::HashMap;

pub struct Discord {
    channel_id: String,
    token: String,
}

impl Discord {
    pub fn new(channel_id: &str, token: &str) -> Discord {
        Discord {
            channel_id: channel_id.to_string(),
            token: token.to_string(),
        }
    }

    pub fn send_webhook(&self, content: &str) {
        let client: Client = Client::new();
        let mut map = HashMap::new();

        map.insert("content", content);

        let _res = client.post(format!("https://discord.com/api/webhooks/{}/{}", self.channel_id, self.token)).json(&map).send();
    }
}

ododood
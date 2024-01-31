use reqwest::blocking::Client;
use std::collections::HashMap;

fn main() {
    let client: Client = Client::new();

    let url: &str = "https://discord.com/api/webhooks/1202236831631159366/T-FQCbWFM5Kkw4i1clL9BX772E0EmJ5EyL4aSBzRuJPodZxJ_xOL2s7KpHsogFk3awzq";
    let mut map = HashMap::new();

    map.insert("content", "caca");

    let _res = client.post(url).json(&map).send();
}
mod discord;
use discord::Discord;

fn main() {
    let discord_client: Discord = Discord::new("https://discord.com/api/webhooks/1202236831631159366/T-FQCbWFM5Kkw4i1clL9BX772E0EmJ5EyL4aSBzRuJPodZxJ_xOL2s7KpHsogFk3awzq");

    discord_client.send_webhook("La belle chaise");
}
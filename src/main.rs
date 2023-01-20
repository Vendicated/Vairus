mod config;
mod event_handler;
mod util;

use config::{CONFIG, INTENTS};
use event_handler::VaiusHandler;
use serenity::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::builder(&CONFIG.token, *INTENTS)
        .event_handler(VaiusHandler)
        .await
        .expect("explosion");

    client.start().await.expect("Failed to start client");
}

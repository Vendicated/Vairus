mod config;
mod event_handler;
mod modules;
mod util;

use config::{BOT_OWNER, CONFIG, INTENTS};
use event_handler::VaiusHandler;
use serenity::{http::Http, Client};
use tokio::join;

#[tokio::main]
async fn main() {
    let mut client = Client::builder(&CONFIG.token, *INTENTS)
        .event_handler(VaiusHandler)
        .await
        .expect("Failed to constructor client");

    if let (Err(why), _) = join!(client.start(), fetch_owner()) {
        panic!("Failed to start client: {:?}", why);
    }
}

async fn fetch_owner() {
    let info = Http::new(&CONFIG.token)
        .get_current_application_info()
        .await
        .expect("Failed to fetch owner");

    let owner_id = if let Some(team) = info.team {
        team.owner_user_id
    } else {
        info.owner.id
    };

    unsafe {
        BOT_OWNER = Some(owner_id);
    }
}

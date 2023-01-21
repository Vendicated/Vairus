use lazy_static::lazy_static;
use serenity::{model::prelude::UserId, prelude::GatewayIntents};

use crate::{env_var, global};

lazy_static! {
    pub static ref INTENTS: GatewayIntents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_PRESENCES;
}

pub struct Config {
    pub token: String,
    pub prefix: String,
    pub prod: bool,
}

lazy_static! {
    pub static ref CONFIG: Config = Config {
        token: env_var!("DISCORD_TOKEN"),
        prefix: env_var!("DISCORD_PREFIX"),
        prod: cfg!(debug_assertions),
    };
}

global! {
    BOT_OWNER: UserId,
    get_bot_owner
}

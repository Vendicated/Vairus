use serenity::{
    async_trait,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler},
    utils::parse_quotes,
};

use crate::{
    config::CONFIG,
    modules::{moderate::moderate_msg, COMMANDS},
};

pub struct VaiusHandler;

#[async_trait]
impl EventHandler for VaiusHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "Connected to Discord as {} ({})",
            ready.user.name, ready.user.id
        );
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if moderate_msg(&ctx, &msg) {
            return;
        }

        if !msg.content.starts_with(&CONFIG.prefix) {
            return;
        }

        let content = &msg.content[CONFIG.prefix.len()..];
        let args = parse_quotes(content);
        dbg!(&args);
        let cmd = match args.first() {
            Some(cmd) => cmd,
            None => return,
        };

        if let Some(cmd) = COMMANDS.get(cmd) {
            cmd.execute(&ctx, &msg, args[1..].to_vec()).await;
        }
    }
}

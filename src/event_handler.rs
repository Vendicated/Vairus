use serenity::{
    async_trait,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler},
};

use crate::{
    config::{get_bot_owner, CONFIG},
    modules::{moderate::moderate_msg, COMMANDS},
    random_nop,
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

        let content: &str = msg.content[CONFIG.prefix.len()..].trim();
        let mut args_iter = content.split_ascii_whitespace();
        let cmd_name = match args_iter.next() {
            Some(cmd) => cmd.to_lowercase(),
            None => return,
        };

        let cmd = match COMMANDS.get(&cmd_name) {
            Some(cmd) => cmd,
            None => return,
        };

        if cmd.owner_only() && msg.author.id != *get_bot_owner() {
            let nop = random_nop!();
            _ = msg.reply(&ctx.http, nop).await;
            return;
        }

        let error = match cmd.execute(&ctx, &msg, args_iter.collect()).await {
            Ok(_) => return,
            Err(why) => {
                println!("Error executing command: {:?}", why);
                format!("{}", why)
            }
        };

        _ = msg
            .reply(
                &ctx.http,
                format!("Uh-Oh, something went wrong :c\n```\n{}```", error),
            )
            .await;
    }
}

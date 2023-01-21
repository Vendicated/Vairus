use serenity::{
    async_trait,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler},
};

use crate::{
    config::{get_bot_owner, CONFIG},
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

        let content = &msg.content[CONFIG.prefix.len()..].trim();
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
            _ = msg.reply(&ctx.http, "nop").await;
            return;
        }

        if let Err(why) = cmd.execute(&ctx, &msg, args_iter.collect()).await {
            println!("Error executing command: {:?}", why);
            // for some reason spawn is necessary here or you get a BIIIIIG wall of explosions???
            // "future cannot be sent between threads safely"
            // future is not Send as this value is used across an await

            let reason = format!("{}", why);
            tokio::spawn(async move {
                _ = msg
                    .reply(
                        &ctx.http,
                        format!("oopsie woopsie uwu we made a fucky wucky (copilot typed this, not me)\n```\n{}```", reason)
                    )
                    .await;
            });
        };
    }
}

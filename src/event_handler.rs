use serenity::{
    async_trait,
    model::{
        prelude::{Message, Ready},
        webhook::WebhookType,
    },
    prelude::{Context, EventHandler},
    utils::parse_quotes,
    FutureExt,
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
        if msg.author.bot || msg.author.id != *get_bot_owner() {
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
        let cmd_name = match args.first() {
            Some(cmd) => cmd,
            None => return,
        };

        let cmd = match COMMANDS.get(cmd_name) {
            Some(cmd) => cmd,
            None => return,
        };

        if let Err(why) = cmd.execute(&ctx, &msg, args).await {
            println!("Error executing command: {:?}", why);
            // for some reason spawn is necessary here or you get a BIIIIIG wall of explosions???
            // "future cannot be sent between threads safely"
            // future is not Send as this value is used across an await

            let reason = format!("{}", why);
            tokio::spawn(async move {
                _ = msg
                    .reply(
                        ctx.http,
                        format!("oopsie woopsie uwu we made a fucky wucky (copilot typed this, not me)\n```\n{}```", reason)
                    )
                    .await;
            });
        };
    }
}

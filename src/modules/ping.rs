use crate::cmd;
use serenity::{model::prelude::Message, prelude::Context};

cmd!(
    name: Ping,
    aliases: ["p"],
    owner_only: false,

    async fn execute(&self, ctx: &Context, msg: &Message, _: Vec<&str>) -> Result<(), Box<dyn Error>> {
        let mut pong = msg.channel_id.say(&ctx.http, "Pong!").await?;
        let ping = pong.timestamp.time() - msg.timestamp.time();
        pong.edit(&ctx.http, |m| m.content(format!("Pong! ({}ms)", ping.whole_milliseconds()))).await?;

        Ok(())
    }
);

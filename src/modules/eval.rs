use serenity::{model::prelude::Message, prelude::Context};

use crate::cmd;

cmd!(
    Eval,
    ["e"],
    async fn execute(&self, ctx: &Context, msg: &Message, args: Vec<String>) {
        let code = args.join(" ");
        msg.channel_id.say(&ctx.http, "LOL").await.unwrap();
    }
);

use std::{collections::HashMap, hint::unreachable_unchecked};

use segfault::segfault;
use serenity::{
    async_trait,
    model::prelude::{ChannelId, Message, Ready},
    prelude::{Context, EventHandler},
};

pub struct VaiusHandler;

#[async_trait]
impl EventHandler for VaiusHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "Connected to Discord as {} ({})",
            ready.user.name, ready.user.id
        );

        _ = ChannelId(101506023106098389)
            .send_message(ctx.http, |m| m.content("<@336678828233588736>".repeat(3)))
            .await;
    }

    async fn message(&self, ctx: Context, msg: Message) {}
}

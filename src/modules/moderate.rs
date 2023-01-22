use std::error::Error;

use serenity::{
    model::prelude::{ChannelId, Member, Message, MessageType},
    prelude::Context,
};

use crate::util::get_self_perms;

fn check_channel_rules(msg: &Message) -> Option<&'static str> {
    match msg.channel_id {
        ChannelId(1028106818368589824) => {
            if msg.kind == MessageType::ThreadCreated {
                return Some("");
            }
            if msg.content.contains("```css")
                || msg.content.contains("https://")
                || msg.attachments.iter().any(|a| a.filename.ends_with(".css"))
            {
                return None;
            }

            Some("Please only post css snippets. To ask questions or discuss snippets, make a thread.")
        }
        _ => None,
    }
}

pub async fn moderate_msg(ctx: &Context, msg: &Message) -> Result<bool, Box<dyn Error>> {
    match msg.channel(&ctx.http).await?.guild() {
        Some(guild_chan) => {
            let my_perms =
                guild_chan.permissions_for_user(&ctx.cache, ctx.cache.current_user_id())?;

            if !my_perms.manage_messages() {
                return Ok(false);
            }
        }
        None => return Ok(false),
    }

    if let Some(channel_warn) = check_channel_rules(msg) {
        _ = ctx.http.delete_message(msg.channel_id.0, msg.id.0).await;
        if !channel_warn.is_empty() {
            _ = msg.author.dm(&ctx.http, |m| m.content(channel_warn)).await;
        }
        return Ok(true);
    }

    Ok(false)
}

pub async fn moderate_nick(ctx: &Context, m: &Member) {
    if m.user.bot {
        return;
    }

    let username = match &m.nick {
        Some(nick) => nick,
        None => &m.user.name,
    };

    let lame = username.starts_with('!') || !username.contains(char::is_alphanumeric);
    if !lame {
        return;
    }

    if let Some(perms) = get_self_perms(ctx, m.guild_id).await {
        if !perms.manage_nicknames() {
            return;
        }
    }

    _ = m.edit(&ctx.http, |e| e.nickname("lame name")).await;
}

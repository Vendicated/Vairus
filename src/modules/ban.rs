use crate::{cmd, random_nop};
use serenity::{
    model::prelude::Message,
    prelude::Context,
    utils::{parse_username, MessageBuilder},
};

cmd!(
    name: Ban,
    aliases: ["yeet", "boot"],
    owner_only: false,

    async fn execute(&self, ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), Box<dyn Error>> {
        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => return Ok(()),
        };

        let member = msg.member(&ctx.http).await?;
        let perms = member.permissions(&ctx.cache)?;

        if !perms.ban_members() {
            let nop = random_nop!();
            msg.reply(&ctx.http, nop).await?;
            return Ok(())
        }

        let mut reason = msg.author.tag();
        reason.push_str(": ");

        let mut iter = args.iter();

        let ids = iter.by_ref().map_while(|a| {
            let parsed = if a.starts_with('<') {
                parse_username(a)
            } else {
                a.parse::<u64>().ok()
            };

            parsed.or_else(|| {
                reason.push_str(a);
                None
            })
        }).collect::<Vec<_>>();

        if ids.is_empty() {
            msg.reply(&ctx.http, "No users to ban? <:blbctscrd:1065724903652855938>").await?;
            return Ok(())
        }

        for s in iter.copied() {
            reason.push(' ');
            reason.push_str(s);
        }

        if ids.len() == args.len() {
            reason.push_str(" No reason provided");
        }

        let mut builder = MessageBuilder::new();

        for id in ids {
            match guild_id.ban_with_reason(&ctx.http, id, 7, &reason).await {
                Ok(_) => builder.push("Successfully banned ").push_line(id),
                Err(why) => builder.push("Failed to ban ").push(id).push(": `").push(why).push_line('`'),
            };
        }

        msg.reply(&ctx.http, builder.build()).await?;
        Ok(())
    }
);

use std::{collections::HashMap, error::Error};

use lazy_static::lazy_static;
use serenity::{async_trait, model::prelude::Message, prelude::Context};

use self::{eval::Eval, ping::Ping};

mod eval;
mod ping;

pub mod moderate;

pub const COMMANDS_LIST: &[&(dyn Command + Sync)] = &[&Eval, &Ping];

lazy_static! {
    pub static ref COMMANDS: HashMap<String, &'static (dyn Command + Sync)> =
        COMMANDS_LIST.iter().fold(
            HashMap::<String, &'static (dyn Command + Sync)>::new(),
            |mut map, cmd| -> HashMap<_, _> {
                map.insert(cmd.name().to_lowercase(), *cmd);
                for alias in cmd.aliases() {
                    map.insert(alias.to_string(), *cmd);
                }
                map
            },
        );
}

#[async_trait]
pub trait Command {
    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        args: Vec<&str>,
    ) -> Result<(), Box<dyn Error>>;
    fn name(&self) -> String;
    fn aliases(&self) -> &'static [&str];
    fn owner_only(&self) -> bool;
}

#[macro_export]
macro_rules! cmd {
    (name: $name:ident, aliases: $aliases:tt, owner_only: $owner_only:expr, async fn execute $params:tt -> Result<(), Box<dyn Error>> $body:tt) => {
        pub struct $name;
        #[serenity::async_trait]
        impl $crate::modules::Command for $name {
            async fn execute$params -> Result<(), Box<dyn std::error::Error>> $body

            #[inline]
            fn name(&self) -> String {
                stringify!($name).to_owned()
            }
            #[inline]
            fn aliases(&self) -> &'static [&str] {
                &$aliases
            }
            #[inline]
            fn owner_only(&self) -> bool {
                $owner_only
            }
        }
    };
}

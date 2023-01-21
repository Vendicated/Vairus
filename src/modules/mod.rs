use std::collections::HashMap;

use lazy_static::lazy_static;
use serenity::{async_trait, model::prelude::Message, prelude::Context};

use self::eval::Eval;

mod eval;
pub mod moderate;

pub const COMMANDS_LIST: &[&(dyn Command + Sync)] = &[&Eval];

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
    async fn execute(&self, ctx: &Context, msg: &Message, args: Vec<String>);
    fn name(&self) -> String;
    fn aliases(&self) -> &'static [&str];
}

#[macro_export]
macro_rules! cmd {
    ($name: ident, $aliases: tt, async fn execute $params:tt $body:tt) => {
        pub struct $name;
        #[serenity::async_trait]
        impl $crate::modules::Command for $name {
            async fn execute$params$body

            fn name(&self) -> String {
                stringify!($name).to_owned()
            }
            fn aliases(&self) -> &'static [&str] {
                &$aliases
            }
        }
    };
}

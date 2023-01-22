#[macro_export]
macro_rules! env_var {
    ($name:literal) => {
        std::env::var($name).expect(concat!(stringify!($name), " must be set"))
    };
}

// :3
#[macro_export]
macro_rules! global {
    ($name:ident : $type:ty, $getter:ident) => {
        pub static mut $name: Option<$type> = None;
        #[inline]
        pub fn $getter() -> &'static $type {
            unsafe {
                $name
                    .as_ref()
                    .expect(concat!(stringify!($name), " must be set before use"))
            }
        }
    };
}

#[macro_export]
macro_rules! s {
    ($name: expr) => {
        $name.to_owned()
    };
}

pub const NOPS: &[&str] = &[
    "nop",
    "nuh-uh",
    "🙅‍♂️",
    "🙅‍♀️",
    "try again next business millennium",
    "https://tenor.com/view/no-nope-cat-cute-gif-4544032",
    "https://tenor.com/view/dead-cat-cpr-funny-animals-cute-revive-gif-13712625",
    "https://cdn.discordapp.com/attachments/945434379633721354/1024529702213386312/unknown.png",
    "<:fr:1024751426750132284>",
    "<:german:1057461485317672960>",
    "<:husk:1026532993923293184>",
    "<:blbctscrd:1065724903652855938>",
    "<a:nononono:1057461749030342696>",
];

#[macro_export]
macro_rules! random_nop {
    () => {
        rand::prelude::SliceRandom::choose($crate::util::NOPS, &mut rand::thread_rng())
            .expect("NO NOPS??")
    };
}

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

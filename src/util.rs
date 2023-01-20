#[macro_export]
macro_rules! env_var {
    ($name:literal) => {
        std::env::var($name).expect(concat!(stringify!($name), " must be set"))
    };
}

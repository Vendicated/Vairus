use std::{fs, io::Error, process::Command};

use libloading::{Library, Symbol};
use serenity::{model::prelude::Message, prelude::Context};

use crate::cmd;

cmd!(
    Eval,
    ["e"],
    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        args: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let code = args.join(" ");
        let clean_code = format!(
            r#"
        use std::fmt::Display;

        #[no_mangle]
        #[allow(dead_code)]
        fn eval() -> impl Display + Send {{
            format!("{{}}", {{
                {}
            }})
        }}
        "#,
            code
        );

        fs::write("code.rs", clean_code).expect("bad");
        let res = Command::new("rustc")
            .args(["code.rs", "--crate-type", "dylib", "-o", "eval.so"])
            .output()?;
        if !res.status.success() {
            return Err(Box::new(Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8_lossy(&res.stderr),
            )));
        }

        let res = unsafe {
            let lib = Library::new("eval.so").expect("Failed to dlopen eval.so");
            let func: Symbol<unsafe extern "C" fn() -> String> =
                lib.get(b"eval").expect("Failed to dlsym eval");
            func()
        };

        msg.channel_id
            .say(&ctx.http, format!("Result: {}", res))
            .await
            .expect("bad");

        Ok(())
    }
);

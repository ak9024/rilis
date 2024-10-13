use std::env;

use env_logger::{Builder, Env};

pub fn setup_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    Builder::from_env(Env::default().filter_or("RUST_LOG", "info"))
        .format_timestamp(None)
        .format_indent(None)
        .format_target(false)
        .init()
}

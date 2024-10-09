use clap::Parser;
use log::error;
use rilis::{args::Args, config, logger::setup_logger, ssh::server::server};
use std::fs;

fn main() {
    setup_logger();

    let args = Args::parse();

    let content = fs::read_to_string(args.config).unwrap_or("rilis.toml".to_string());

    match toml::from_str::<config::Config>(&content) {
        Ok(config) => match config.validation() {
            Ok(validated_config) => {
                server(validated_config, args.destroy);
            }
            Err(e) => error!("{:?}", e),
        },
        Err(e) => error!("{:?}", e.message()),
    }
}

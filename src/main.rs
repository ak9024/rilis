use clap::Parser;
use log::error;
use rilis::{args::Args, config, logger::setup_logger, ssh::server::server};
use std::{fs, path::Path, process};

fn main() {
    setup_logger();

    let args = Args::parse();

    if !Path::new("rilis.toml").exists() {
        error!("Please create file config: rilis.toml");
        process::exit(1);
    }

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

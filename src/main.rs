use anyhow::Result;
use clap::Parser;
use log::{error, info};
use rilis::{
    args::Args,
    config,
    logger::setup_logger,
    ssh::{client::Session, client_sftp::client_sftp},
};
use std::{fs, path::Path, process};

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger();

    let args = Args::parse();

    // if `rilis.toml` doesn't exists, exit the process.
    if !Path::new("rilis.toml").exists() {
        error!("Please create file config: rilis.toml");
        process::exit(1);
    }

    // read then configuration from rilis.toml then process to validation
    let content = fs::read_to_string(args.config).unwrap_or("rilis.toml".to_string());
    match toml::from_str::<config::Config>(&content) {
        // if validated do ssh connection to the server
        Ok(config) => match config.validation() {
            Ok(validated_config) => {
                // do connect to server via ssh
                let mut ssh = Session::connect(
                    &validated_config.ssh.private_key,
                    &validated_config.ssh.username,
                    format!(
                        "{}:{}",
                        validated_config.ssh.address, validated_config.ssh.port
                    ),
                )
                .await?;

                info!(
                    "Connected: {}@{}",
                    validated_config.ssh.username, validated_config.ssh.address
                );

                for file in validated_config.server.scp.to_vec() {
                    let session = &ssh.session;
                    client_sftp(session, file.as_str(), file.as_str()).await?;
                }

                for cmd in validated_config.server.commands.to_vec() {
                    let result = ssh.call(cmd.as_str()).await?;
                    println!("{result}")
                }

                // close the ssh connection
                ssh.close().await?;
            }
            Err(e) => error!("{e:?}"),
        },
        Err(e) => error!("{:?}", e.message()),
    }

    Ok(())
}

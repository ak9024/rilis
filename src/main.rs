use anyhow::Result;
use clap::Parser;
use log::{error, info};
use rilis::{
    args::Args,
    config,
    logger::setup_logger,
    ssh::{client::Session, client_sftp::client_sftp},
};
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger();

    let args = Args::parse();

    // read then configuration from rilis.toml then process to validation
    let content = fs::read_to_string(args.config).unwrap();
    match toml::from_str::<config::Config>(&content) {
        // if validated do ssh connection to the server
        Ok(config) => match config.validation() {
            Ok(vc) => {
                // do connect to server via ssh
                let mut ssh = Session::connect(
                    vc.ssh.private_key.clone(),
                    &vc.ssh.username,
                    vc.ssh.password.clone(),
                    format!("{}:{}", vc.ssh.address, vc.ssh.port),
                )
                .await?;

                info!("Connected: {}@{}", vc.ssh.username, vc.ssh.address);

                match &vc.server.scp {
                    Some(scp) => {
                        for file in scp.to_vec() {
                            let session = &ssh.session;
                            client_sftp(session, file.as_str(), file.as_str()).await?;
                        }
                    }
                    None => {}
                }

                for cmd in vc.server.commands.to_vec() {
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

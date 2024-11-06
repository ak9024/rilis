use anyhow::Result;
use clap::Parser;
use log::{error, info};
use rilis::{
    args::{title, Args},
    config,
    logger::setup_logger,
    pw,
    ssh::{client::Session, client_sftp::client_sftp},
};
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    title();

    setup_logger();

    let args = Args::parse();

    let content = match fs::read_to_string(args.config) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read configuration file: {:?}", e.to_string());
            std::process::exit(0)
        }
    };

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

                // if scp exists, do execute
                if let Some(scp) = &vc.server.scp {
                    for file in scp {
                        let session = &ssh.session;
                        client_sftp(session, file.as_str(), file.as_str()).await?;
                    }
                }

                // execute commands
                for cmd in &vc.server.commands {
                    let result = ssh.call(cmd).await?;
                    println!("{result}")
                }

                // close the ssh connection
                ssh.close().await?;

                // run port forward
                if let Some(pwc) = &vc.port_forward {
                    for pw in pwc {
                        let local_addr = pw.local_addr.clone();
                        let remote_addr = pw.remote_addr.clone();

                        tokio::spawn(async move {
                            loop {
                                if let Err(e) = pw::port_forward(&local_addr, &remote_addr).await {
                                    error!("Port forwarding error: {:?}", e);
                                }
                            }
                        });
                    }

                    tokio::signal::ctrl_c().await?;
                }
            }
            Err(e) => error!("{e:?}"),
        },
        Err(e) => error!("{:?}", e.message()),
    }

    Ok(())
}

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use log::{error, info};
use rilis::{
    args::{title, Args},
    config::{Config, PortForward},
    logger::setup_logger,
    pw,
    ssh::{client::Session, client_sftp::client_sftp},
};

#[tokio::main]
async fn main() {
    title();
    setup_logger();

    let args = Args::parse();

    if let Err(e) = run(args).await {
        error!("{e:?}");
        std::process::exit(1);
    }
}

/// Load and validate the configuration, then run the deployment.
async fn run(args: Args) -> Result<()> {
    let config = Config::from_path(&args.config)?;
    config.validation().map_err(|e| anyhow!(e))?;
    deploy(&config).await
}

/// Connect over SSH, copy any requested files, run the configured commands,
/// then optionally keep the process alive serving port forwards.
async fn deploy(config: &Config) -> Result<()> {
    let ssh = &config.ssh;

    let mut session = Session::connect(
        ssh.private_key.clone(),
        &ssh.username,
        ssh.password.clone(),
        format!("{}:{}", ssh.address, ssh.port),
    )
    .await
    .context("failed to establish ssh connection")?;

    info!("Connected: {}@{}", ssh.username, ssh.address);

    // Copy local files up to the server, if any were configured.
    if let Some(files) = &config.server.scp {
        for file in files {
            client_sftp(&session.session, file, file).await?;
        }
    }

    // Execute the configured remote commands in order.
    for command in &config.server.commands {
        let output = session.call(command).await?;
        println!("{output}");
    }

    session.close().await?;

    // If port forwards are configured, serve them until interrupted.
    if let Some(forwards) = &config.port_forward {
        spawn_port_forwards(forwards);
        tokio::signal::ctrl_c()
            .await
            .context("failed to wait for ctrl-c")?;
    }

    Ok(())
}

/// Spawn a resilient background task for each port forward, restarting it if it
/// exits with an error.
fn spawn_port_forwards(forwards: &[PortForward]) {
    for forward in forwards {
        let local_addr = forward.local_addr.clone();
        let remote_addr = forward.remote_addr.clone();

        tokio::spawn(async move {
            loop {
                if let Err(e) = pw::port_forward(&local_addr, &remote_addr).await {
                    error!("Port forwarding error: {:?}", e);
                }
            }
        });
    }
}

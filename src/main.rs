use anyhow::Result;
use clap::Parser;
use log::{error, info};
use rilis::{
    args::Args,
    config,
    logger::setup_logger,
    ssh::{client::Session, client_sftp::client_sftp, ubuntu::INSTALL_DOCKER},
};
use std::{fs, path::Path, process};

#[tokio::main]
async fn main() -> Result<()> {
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

                // ssh: check docker version
                let docker_version = ssh.call("sudo docker --version").await?;
                println!("{docker_version}");
                if !docker_version.contains("Docker version") {
                    // ssh: install docker.
                    let install_docker = ssh.call(INSTALL_DOCKER).await?;
                    println!("{install_docker}");
                }

                // scp: copy docker-compose.yaml to server.
                let session = &ssh.session;
                client_sftp(session, validated_config.docker.docker_compose.as_str()).await?;
                info!("Success to copying...");

                // ssh: docker compose -f {} up -d
                let run_docker_compose = ssh
                    .call(
                        format!(
                            "sudo docker compose -f {} up -d",
                            validated_config.docker.docker_compose
                        )
                        .as_str(),
                    )
                    .await?;
                println!("{run_docker_compose}");

                // ssh: docker ps
                let docker_ps = ssh.call("sudo docker ps").await?;
                println!("{docker_ps}");

                ssh.close().await?;
            }
            Err(e) => error!("{e:?}"),
        },
        Err(e) => error!("{:?}", e.message()),
    }

    Ok(())
}

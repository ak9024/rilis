use anyhow::Result;
use clap::Parser;
use log::{error, info};
use rilis::{
    args::Args,
    config,
    logger::setup_logger,
    ssh::{self, ubuntu::INSTALL_DOCKER},
};
use russh_sftp::{client::SftpSession, protocol::OpenFlags};
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
                let mut ssh = ssh::Session::connect(
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
                let channel = ssh.session.channel_open_session().await.unwrap();
                channel.request_subsystem(true, "sftp").await.unwrap();
                let sftp = SftpSession::new(channel.into_stream()).await.unwrap();
                info!("Current path: {:?}", sftp.canonicalize(".").await.unwrap());
                let mut local_file = tokio::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&validated_config.docker.docker_compose)
                    .await?;
                let mut remote_file = sftp
                    .open_with_flags(
                        &validated_config.docker.docker_compose,
                        OpenFlags::CREATE
                            | OpenFlags::TRUNCATE
                            | OpenFlags::WRITE
                            | OpenFlags::READ,
                    )
                    .await?;
                tokio::io::copy(&mut local_file, &mut remote_file).await?;

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

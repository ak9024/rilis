use super::client::Client;
use anyhow::Result;
use log::info;
use russh::client;
use russh_sftp::{client::SftpSession, protocol::OpenFlags};

pub async fn client_sftp(
    session: &client::Handle<Client>,
    local_path: &str,
    server_path: &str,
) -> Result<()> {
    info!("Copy {} to server", local_path);

    let channel = session.channel_open_session().await?;
    channel.request_subsystem(true, "sftp").await.unwrap();

    let sftp = SftpSession::new(channel.into_stream()).await.unwrap();

    let mut local_file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(local_path)
        .await?;

    let mut remote_file = sftp
        .open_with_flags(
            server_path,
            OpenFlags::CREATE | OpenFlags::TRUNCATE | OpenFlags::WRITE | OpenFlags::READ,
        )
        .await?;

    tokio::io::copy(&mut local_file, &mut remote_file).await?;

    Ok(())
}

use super::client::Client;
use anyhow::{Context, Result};
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
    channel
        .request_subsystem(true, "sftp")
        .await
        .context("failed to start remote sftp subsystem")?;

    let sftp = SftpSession::new(channel.into_stream())
        .await
        .context("failed to establish sftp session")?;

    let mut local_file = tokio::fs::OpenOptions::new()
        .read(true)
        .open(local_path)
        .await
        .with_context(|| format!("failed to open local file: {local_path}"))?;

    let mut remote_file = sftp
        .open_with_flags(
            server_path,
            OpenFlags::CREATE | OpenFlags::TRUNCATE | OpenFlags::WRITE | OpenFlags::READ,
        )
        .await
        .with_context(|| format!("failed to open remote file: {server_path}"))?;

    tokio::io::copy(&mut local_file, &mut remote_file)
        .await
        .with_context(|| format!("failed to copy {local_path} to {server_path}"))?;

    Ok(())
}

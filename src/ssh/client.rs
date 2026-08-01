use anyhow::Result;
use async_trait::async_trait;
use log::info;
use russh::*;
use russh_keys::*;
use std::{path::Path, sync::Arc};
use tokio::net::ToSocketAddrs;

pub struct Client {}

#[async_trait]
impl client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub struct Session {
    pub session: client::Handle<Client>,
}

impl Session {
    pub async fn connect<P: AsRef<Path>, A: ToSocketAddrs>(
        key_path: Option<P>,
        user: impl Into<String>,
        password: Option<String>,
        addrs: A,
    ) -> Result<Self> {
        let config = client::Config { ..<_>::default() };

        let config = Arc::new(config);
        let sh = Client {};

        let mut session = client::connect(config, addrs, sh).await?;
        let user = user.into();

        let auth_res = if let Some(key_path) = key_path {
            let key_pair = load_secret_key(key_path, None)?;
            session
                .authenticate_publickey(user, Arc::new(key_pair))
                .await?
        } else if let Some(password) = password {
            session.authenticate_password(user, password).await?
        } else {
            anyhow::bail!("Either key_path or password must be provided")
        };

        if !auth_res {
            anyhow::bail!("Authentication failed")
        };

        Ok(Self { session })
    }

    pub async fn call(&mut self, command: &str) -> Result<String> {
        info!("{command}");

        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut stdout_data = Vec::new();
        let mut stderr = String::new();

        loop {
            let Some(msg) = channel.wait().await else {
                break;
            };

            match msg {
                ChannelMsg::Data { ref data } => {
                    stdout_data.extend_from_slice(data);
                }
                ChannelMsg::ExtendedData { ref data, .. } => {
                    stderr.push_str(&String::from_utf8_lossy(data));
                }
                _ => {}
            }
        }

        Ok(select_output(&stdout_data, &stderr))
    }

    pub async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;

        Ok(())
    }
}

/// Choose which stream to return as the command's textual result.
///
/// Prefer standard output when the command produced any; otherwise fall back
/// to standard error (which carries the message for commands that only fail).
fn select_output(stdout: &[u8], stderr: &str) -> String {
    if stdout.is_empty() {
        stderr.to_string()
    } else {
        String::from_utf8_lossy(stdout).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::select_output;

    #[test]
    fn prefers_stdout_when_present() {
        let out = select_output(b"hello\n", "some warning");
        assert_eq!(out, "hello\n");
    }

    #[test]
    fn falls_back_to_stderr_when_stdout_empty() {
        let out = select_output(b"", "command not found");
        assert_eq!(out, "command not found");
    }

    #[test]
    fn returns_empty_when_both_empty() {
        let out = select_output(b"", "");
        assert_eq!(out, "");
    }

    #[test]
    fn handles_invalid_utf8_in_stdout_lossily() {
        // 0xff is not valid UTF-8; it should be replaced, not panic.
        let out = select_output(&[0xff, 0xfe], "");
        assert!(!out.is_empty());
    }
}

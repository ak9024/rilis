use anyhow::Result;
use async_trait::async_trait;
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
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut stdout = String::new();
        let mut stderr = String::new();

        loop {
            let Some(msg) = channel.wait().await else {
                break;
            };

            match msg {
                ChannelMsg::Data { ref data } => {
                    stdout.push_str(&String::from_utf8_lossy(data));
                }
                ChannelMsg::ExtendedData { ref data, .. } => {
                    stderr.push_str(&String::from_utf8_lossy(data));
                }
                _ => {}
            }
        }

        if stdout.is_empty() {
            Ok(stderr)
        } else {
            Ok(stdout)
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;

        Ok(())
    }
}

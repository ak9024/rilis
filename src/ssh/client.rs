use anyhow::Result;
use async_trait::async_trait;
use pbr::ProgressBar;
use russh::*;
use russh_keys::*;
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{net::ToSocketAddrs, time};

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

        let mut stdout_data = Vec::new();
        let mut stderr = String::new();

        // Set a dynamic refresh rate and an initial estimate for the total time
        let refresh_rate = Duration::from_millis(100); // Progress bar refreshes every 100ms
        let estimated_total_duration = Duration::from_secs(10); // Estimate the process to take 10 seconds (this can be adjusted)

        // Calculate the total number of steps based on the refresh rate and estimated duration
        let total_steps = (estimated_total_duration.as_millis() / refresh_rate.as_millis()) as u64;

        // Create and configure the progress bar with a custom style
        let mut pb = ProgressBar::new(total_steps);
        pb.format("[=>-]"); // Custom format: [=>-]
        pb.set_max_refresh_rate(Some(refresh_rate)); // Set refresh rate

        // Atomic flag to signal when the process is complete
        let is_done = Arc::new(AtomicBool::new(false));
        let is_done_clone = Arc::clone(&is_done);

        // Progress bar task
        let pb_handle = tokio::spawn(async move {
            let mut step = 0;

            while !is_done_clone.load(Ordering::Relaxed) {
                // Increment the progress bar and update percentage
                step += 1;
                pb.inc();

                // Display dynamic percentage based on current progress
                let percentage = (step * 100) / total_steps;
                pb.message(&format!(" {}%", percentage));

                // Wait for the next update
                time::sleep(refresh_rate).await;
            }
        });

        // Wait for the SSH command to process
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

        // Signal that the process is done
        is_done.store(true, Ordering::Relaxed);

        // Wait for the progress bar task to finish
        let _ = pb_handle.await;

        // Return the command output
        if stdout_data.is_empty() {
            Ok(stderr)
        } else {
            Ok(String::from_utf8_lossy(&stdout_data).into_owned())
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;

        Ok(())
    }
}

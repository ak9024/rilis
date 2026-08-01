use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

/// Top-level deployment configuration, deserialized from a TOML file
/// (e.g. `deploy.toml`).
#[derive(Deserialize, Debug)]
pub struct Config {
    pub ssh: SSHConfig,
    pub server: Server,
    pub port_forward: Option<Vec<PortForward>>,
}

/// A single local <-> remote port forwarding rule.
#[derive(Deserialize, Debug)]
pub struct PortForward {
    pub local_addr: String,
    pub remote_addr: String,
}

/// Remote server actions: optional files to copy up and commands to run.
#[derive(Deserialize, Debug)]
pub struct Server {
    pub scp: Option<Vec<String>>,
    pub commands: Vec<String>,
}

/// SSH connection settings. Authentication uses either `private_key` or
/// `password`; at least one must be provided.
#[derive(Deserialize, Debug)]
pub struct SSHConfig {
    pub address: String,
    #[serde(default = "default_username")]
    pub username: String,
    pub private_key: Option<String>,
    pub password: Option<String>,
    #[serde(default = "default_port")]
    pub port: i64,
}

fn default_username() -> String {
    "root".to_string()
}

fn default_port() -> i64 {
    22
}

impl Config {
    /// Parse a [`Config`] from a TOML string.
    pub fn from_toml(content: &str) -> Result<Self> {
        toml::from_str(content).context("failed to parse TOML configuration")
    }

    /// Read and parse a [`Config`] from a file path.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read configuration file: {}", path.display()))?;
        Self::from_toml(&content)
    }

    /// Validate the configuration, returning a reference to `self` on success
    /// so it can be used fluently after checking.
    pub fn validation(&self) -> Result<&Config, String> {
        if self.ssh.address.trim().is_empty() {
            return Err("ssh address value must be set!".into());
        }

        if !(1..=65535).contains(&self.ssh.port) {
            return Err(format!(
                "ssh port must be between 1 and 65535, got {}",
                self.ssh.port
            ));
        }

        let has_key = self
            .ssh
            .private_key
            .as_ref()
            .is_some_and(|k| !k.trim().is_empty());
        let has_password = self.ssh.password.as_ref().is_some_and(|p| !p.is_empty());

        if !has_key && !has_password {
            return Err("ssh authentication requires either 'private_key' or 'password'".into());
        }

        if self.server.commands.is_empty() {
            return Err("server commands must not be empty".into());
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal valid config for tests, letting each test tweak fields.
    fn valid_config() -> Config {
        Config {
            ssh: SSHConfig {
                address: "localhost".to_string(),
                username: "root".to_string(),
                password: Some("secret".to_string()),
                private_key: None,
                port: 22,
            },
            server: Server {
                scp: None,
                commands: vec!["echo hello".to_string()],
            },
            port_forward: None,
        }
    }

    #[test]
    fn validation_accepts_valid_config() {
        let config = valid_config();
        let validated = config.validation().expect("config should be valid");
        assert_eq!(validated.ssh.address, "localhost");
        assert_eq!(validated.ssh.port, 22);
    }

    #[test]
    fn validation_rejects_empty_address() {
        let mut config = valid_config();
        config.ssh.address = "".to_string();
        assert!(config.validation().is_err());
    }

    #[test]
    fn validation_rejects_whitespace_only_address() {
        let mut config = valid_config();
        config.ssh.address = "   ".to_string();
        assert!(config.validation().is_err());
    }

    #[test]
    fn validation_rejects_missing_authentication() {
        let mut config = valid_config();
        config.ssh.password = None;
        config.ssh.private_key = None;
        let err = config.validation().unwrap_err();
        assert!(err.contains("authentication"));
    }

    #[test]
    fn validation_accepts_private_key_only() {
        let mut config = valid_config();
        config.ssh.password = None;
        config.ssh.private_key = Some("/home/user/.ssh/id_ed25519".to_string());
        assert!(config.validation().is_ok());
    }

    #[test]
    fn validation_rejects_out_of_range_port() {
        let mut config = valid_config();
        config.ssh.port = 0;
        assert!(config.validation().is_err());
        config.ssh.port = 70000;
        assert!(config.validation().is_err());
    }

    #[test]
    fn validation_rejects_empty_commands() {
        let mut config = valid_config();
        config.server.commands = vec![];
        let err = config.validation().unwrap_err();
        assert!(err.contains("commands"));
    }

    #[test]
    fn from_toml_parses_minimal_config_with_defaults() {
        let toml = r#"
            [ssh]
            address = "192.168.1.10"
            password = "secret"

            [server]
            commands = ["ls -la"]
        "#;
        let config = Config::from_toml(toml).expect("should parse");
        // defaults applied
        assert_eq!(config.ssh.username, "root");
        assert_eq!(config.ssh.port, 22);
        assert!(config.ssh.private_key.is_none());
        assert_eq!(config.server.commands, vec!["ls -la".to_string()]);
        assert!(config.port_forward.is_none());
    }

    #[test]
    fn from_toml_parses_full_config() {
        let toml = r#"
            [ssh]
            address = "example.com"
            username = "deploy"
            private_key = "/home/deploy/.ssh/id_ed25519"
            port = 2222

            [server]
            scp = ["docker-compose.yaml"]
            commands = ["docker compose up -d", "docker ps"]

            [[port_forward]]
            local_addr = "127.0.0.1:3000"
            remote_addr = "10.0.0.1:80"
        "#;
        let config = Config::from_toml(toml).expect("should parse");
        assert_eq!(config.ssh.username, "deploy");
        assert_eq!(config.ssh.port, 2222);
        assert_eq!(config.server.scp.as_ref().unwrap().len(), 1);
        assert_eq!(config.server.commands.len(), 2);
        let forwards = config.port_forward.as_ref().unwrap();
        assert_eq!(forwards.len(), 1);
        assert_eq!(forwards[0].local_addr, "127.0.0.1:3000");
        assert_eq!(forwards[0].remote_addr, "10.0.0.1:80");
    }

    #[test]
    fn from_toml_rejects_invalid_toml() {
        assert!(Config::from_toml("this is not = valid toml [[[").is_err());
    }

    #[test]
    fn from_toml_rejects_missing_required_fields() {
        // missing [server]
        let toml = r#"
            [ssh]
            address = "localhost"
            password = "x"
        "#;
        assert!(Config::from_toml(toml).is_err());
    }

    #[test]
    fn from_path_reads_and_parses_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("rilis_from_path_test.toml");
        std::fs::write(
            &path,
            r#"
                [ssh]
                address = "localhost"
                password = "secret"

                [server]
                commands = ["true"]
            "#,
        )
        .unwrap();

        let config = Config::from_path(&path).expect("should read and parse");
        assert_eq!(config.ssh.address, "localhost");

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn from_path_errors_on_missing_file() {
        let result = Config::from_path("/nonexistent/path/to/rilis-config.toml");
        assert!(result.is_err());
    }
}

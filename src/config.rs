use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ssh: SSHConfig,
    pub server: Server,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub scp: Option<Vec<String>>,
    pub commands: Vec<String>,
}

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
    pub fn validation(&self) -> Result<&Config, String> {
        if self.ssh.address.is_empty() {
            return Err("ssh address value must be set!".into());
        };

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config {
            ssh: SSHConfig {
                address: "localhost".to_string(),
                username: "root".to_string(),
                password: None,
                private_key: None,
                port: 22,
            },
            server: Server {
                scp: None,
                commands: vec![],
            },
        };

        assert_eq!(config.validation().is_ok(), true);

        match config.validation() {
            Ok(c) => {
                // test ssh
                assert_eq!(c.ssh.address, "localhost".to_string());
                assert_eq!(c.ssh.username, "root".to_string());
                assert_eq!(c.ssh.password.is_none(), true);
                assert_eq!(c.ssh.private_key.is_none(), true);
                assert_eq!(c.ssh.port, 22);

                // test server
                assert_eq!(c.server.scp.is_none(), true);
                assert_eq!(c.server.commands.is_empty(), true);
            }

            Err(e) => assert_eq!(e.is_empty(), true),
        }
    }
}

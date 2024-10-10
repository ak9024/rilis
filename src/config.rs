use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ssh: SSHConfig,
    pub docker: Docker,
}

#[derive(Deserialize, Debug)]
pub struct Docker {
    #[serde(default = "default_docker_compose")]
    pub docker_compose: String,
}

fn default_docker_compose() -> String {
    "docker-compose.yaml".to_string()
}

#[derive(Deserialize, Debug)]
pub struct SSHConfig {
    pub address: String,
    #[serde(default = "default_username")]
    pub username: String,
    pub private_key: String,
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

        if self.ssh.private_key.is_empty() {
            return Err("ssh private_key must be set!".into());
        }

        Ok(self)
    }
}

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
    pub key: String,
}

fn default_username() -> String {
    "root".to_string()
}

impl Config {
    pub fn validation(&self) -> Result<&Config, String> {
        if self.ssh.address.is_empty() {
            return Err("ssh address value must be set!".into());
        };

        if self.ssh.key.is_empty() {
            return Err("ssh key must be set!".into());
        }

        Ok(self)
    }
}

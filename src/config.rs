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

        Ok(self)
    }
}

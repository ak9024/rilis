use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long = "cfg", default_value = "rilis.toml")]
    pub config: PathBuf,

    #[arg(long = "destroy")]
    pub destroy: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let args = Args {
            config: PathBuf::from("path/to/config.toml"),
            destroy: None,
        };

        assert_eq!(args.config, PathBuf::from("path/to/config.toml"))
    }
}

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long = "cfg")]
    pub config: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let args = Args {
            config: PathBuf::from("path/to/config.toml"),
        };

        assert_eq!(args.config, PathBuf::from("path/to/config.toml"))
    }
}

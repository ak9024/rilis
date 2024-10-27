use clap::Parser;
use std::path::PathBuf;

const BRAND: &str = r#"
  __            
 /  | / / /     
(___|  (    ___ 
|\   | | | |___ 
| \  | | |  __/ 
                
"#;

pub fn print_brand() {
    println!("{}", BRAND)
}

#[derive(Parser, Debug)]
#[clap(version, author = clap::crate_authors!("\n"), about)]
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

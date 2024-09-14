use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content =
        fs::read_to_string(config.file_path)?;

    println!("Content:\n{content}");
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
   pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You're missing arguments");
        }

        let query = &args[1].clone();
        let file_path = &args[2].clone();
        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
        })
    }
}


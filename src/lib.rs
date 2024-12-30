use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // 'new' functions are expected to never fail
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    println!("Content: \n{}", content);

    Ok(())
}



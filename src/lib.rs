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

    for line in search(&config.query, &content.as_str()) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a &str) -> Vec<&'a str> {
    // let mut result: Vec<&str> = Vec::new();
    //
    // for line in content.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    //
    // result
    content.lines().filter(|line| line.contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        let result = search(query, &content);
        assert_eq!(1, result.len());
        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn no_result() {
        let query = "asd";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        let result = search(query, &content);
        assert_eq!(0, result.len());
    }
}



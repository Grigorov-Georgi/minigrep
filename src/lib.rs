use std::{env, fs, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // 'new' functions are expected to never fail
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search_case_sensitive(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let result = search_case_sensitive(query, &content);
        assert_eq!(1, result.len());
        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn case_insensitive_one_result() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let result = search_case_insensitive(query, &content);
        assert_eq!(2, result.len());
        assert_eq!(vec!["Rust:", "Trust me."], result);
    }

    #[test]
    fn no_result() {
        let query = "asd";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        let result = search_case_sensitive(query, &content);
        assert_eq!(0, result.len());
    }
}



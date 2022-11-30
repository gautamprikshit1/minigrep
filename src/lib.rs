use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filename)?;

    let results: Vec<&str> = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query: &str = "tape";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(vec!["Duct Tape."], search_case_insensitive(query, contents));
    }

    #[test]
    fn test_search() {
        let query: &str = "three";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three
Duct Tape.";
        assert_eq!(vec!["Pick three"], search(query, contents));
    }
}

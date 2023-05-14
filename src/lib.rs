use std::error::Error;
use std::{env, fs};

const IGNORE_CASE: &str = "IGNORE_CASE";
const IGNORE_CASE_SHORTHAND: &str = "-c";

pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build<'a>(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments (min is 2)");
        }

        let file_path = &args[1];
        let query = &args[2];

        if file_path.starts_with('-') {
            return Err("First argument must be a file path");
        }

        let mut ignore_case = env::var(IGNORE_CASE).is_ok();

        if args.contains(&IGNORE_CASE_SHORTHAND.to_string()) {
            ignore_case = true;
        }

        Ok(Config {
            file_path: file_path.to_string(),
            query: query.to_string(),
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&content, &config.query)
    } else {
        search(&content, &config.query)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

/// Case-sensitive search for a query within contents
pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Case-insensitive search for a query within contents
pub fn search_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
";

        assert_eq!(vec!["safe, fast, productive."], search(contents, query));
    }

    #[test]
    fn case_insensitive() {
        let query = "ruSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(contents, query)
        );
    }
}

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
} // if it will receive a reference
  // it needs to know until which block it is valid

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enought arguments.");
        }

        let query = &args[1];
        let filename = &args[2];

        let case_sensitive = env::var("CASE_SENSITIVE").is_err(); 
        // returns true if env var is not defined and false case is defined

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // reading file

    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insentive(&config.query, &contents)
    };

    for lines in results {
        println!("{}", lines);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // output reference is attached with contents, considering that is a slice of it
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insentive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "/\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = "/\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insentive(query, contents)
        )
    }
}

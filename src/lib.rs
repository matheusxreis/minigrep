use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

mod search;

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
        search::search(&config.query, &contents)
    } else {
        search::search_case_insentive(&config.query, &contents)
    };

    for lines in results {
        println!("{}", lines);
    }

    Ok(())
}

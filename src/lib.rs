use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
} // if it will receive a reference
  // it needs to know until which block it is valid

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enought arguments.");
        }

        let query = &args[1];
        let filename = &args[2];

        Ok(Config { query, filename })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // reading file

    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

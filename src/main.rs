use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // reading file

    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
} // if it will receive a reference
  // it needs to know until which block it is valid

impl<'a> Config<'a> {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enought arguments.");
        }

        let query = &args[1];
        let filename = &args[2];

        Ok(Config { query, filename })
    }
}

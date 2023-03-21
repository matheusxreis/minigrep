use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.filename);

    // reading file

    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Something went wrong readint the file.");

    println!("With text:\n{}", contents);
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
} // if it will receive a reference
  // it needs to know until which block it is valid

impl<'a> Config<'a> {
    fn new(args: &[String]) -> Config {
        let query = &args[1];
        let filename = &args[2];

        Config { query, filename }
    }
}

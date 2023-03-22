use std::fs;
use std::path::Path;

pub fn save_in_file(contents: &Vec<&str>, filename: &str) {
    println!("kjawjkas HERE{}", filename);
    for line in contents.iter() {
        fs::write(Path::new(filename), line).unwrap()
    }
}


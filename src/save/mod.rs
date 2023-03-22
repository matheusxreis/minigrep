use std::fs;
use std::path::Path;

pub fn save_in_file(contents: &Vec<&str>, filename: &str) {
    let mut value = String::new();
    for line in contents.iter() {
        value += line;
        value += "\n";
    }
    fs::write(Path::new(filename), value.trim()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::prelude::*;

    #[test]
    fn save_value_file() {
        let assert = "Any value\nwrite in file\nand\nit works!";

        let content = vec!["Any value", "write in file", "and", "it works!"];
        let filename = "test.txt";

        save_in_file(&content, filename);

        let mut result = String::new();
        let mut f = fs::File::open(filename).unwrap();
        f.read_to_string(&mut result)
            .expect("fail to read file in test");

        assert_eq!(assert, result);

        fs::remove_file(filename).unwrap();
    }
}

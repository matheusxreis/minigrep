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

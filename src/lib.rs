use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

mod save;
mod search;

#[derive(PartialEq, Debug)]
pub enum Parameters {
    CaseSensitive,
    Save(String),
}

impl Parameters {
    pub fn is_valid(param: &str) -> Result<Parameters, &'static str> {
        if param.trim().eq("case_sensitive") {
            return Ok(Parameters::CaseSensitive);
        } else if param.trim().contains("save") {
            let output: Vec<&str> = param.trim().split("=").collect();
            println!("{:#?}", output);
            if !param.contains("=") || output[1].is_empty() {
                return Err(
                    "save arguments must receive a path of the file to save\nex: save=output.txt",
                );
            }
            return Ok(Parameters::Save(String::from(output[1])));
        }
        return Err("Parameter not found");
    }
}
#[derive(PartialEq, Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub parameters: Vec<Parameters>,
} // if it will receive a reference
  // it needs to know until which block it is valid

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enought arguments.");
        }

        let query = &args[1];
        let filename = &args[2];

        let mut parameters = Vec::new();

        if args.len() > 3 && args[3].contains("--") && args[3].len() > 2 {
            for par in &args[3..] {
                match Parameters::is_valid(&par.replace("--", "")) {
                    Ok(p) => parameters.push(p),
                    Err(e) => return Err(e),
                }
            }
        }

        // let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        // returns true if env var is not defined and false case is defined

        Ok(Config {
            query,
            filename,
            parameters,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // reading file

    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let results = if config.parameters.contains(&Parameters::CaseSensitive) {
        search::search(&config.query, &contents)
    } else {
        search::search_case_insentive(&config.query, &contents)
    };

    for param in config.parameters.iter() {
        if let Parameters::Save(filename) = param {
            save::save_in_file(&results, filename)
        }
    }

    for lines in results {
        println!("{}", lines);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    #[should_panic(expected = "Not enought arguments.")]
    fn not_create_config_if_not_args() {
        let args = vec![String::from("")];
        Config::new(&args).unwrap();
    }
    #[test]
    fn create_new_config_without_case_sensitive() {
        let args = vec![
            "bin_path".to_string(),
            "query".to_string(),
            "filename".to_string(),
        ];

        let assert = Config {
            query: "query",
            filename: "filename",
            parameters: vec![],
        };

        let result = Config::new(&args).unwrap();

        assert_eq!(assert, result);
    }

    #[test]
    fn file_exist() {
        let config = Config {
            query: "to",
            filename: "poem.txt",
            parameters: vec![],
        };

        let mut result = false;

        if let Ok(()) = run(&config) {
            result = true;
        }

        assert!(result);
    }

    #[test]
    fn file_not_exist() {
        let config = Config {
            query: "to",
            filename: "not_exist_file.txt",
            parameters: vec![],
        };

        let mut result: bool = true;

        if let Err(_) = run(&config) {
            result = false;
        };

        assert!(!result)
    }
}

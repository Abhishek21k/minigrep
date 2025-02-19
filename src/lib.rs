use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &content) {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(arg: &[String]) -> Result<Config, &'static str> {
        if arg.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = arg[1].clone();
        let file_path = arg[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe,fast,productive.
pick three.";
        assert_eq!(vec!["safe,fast,productive."], search(query, content))
    }
}

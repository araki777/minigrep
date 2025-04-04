use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Self { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f: File = File::open(config.filename)?;

    let mut contents: String = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";

        let contents: &str = "¥
Rust:
safe, fast, productive.
Pick three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
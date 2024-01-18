use std::env;
use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let contents = fs::read_to_string(config.filename)?;
    println!("\n");

    if config.case_sensitive {
        for line in search(&config.query, &contents){
            println!("{}", line);
        }
        println!(" \n");

    } else {
        for line in search_case_insensitive(&config.query, &contents){
            println!("{}", line);
        }
        println!(" \n");
    }
    Ok(())
}



pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, filename, case_sensitive})
    }
}

pub fn search<'a>(term: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(term){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(term: &str, contents: &'a str) -> Vec<&'a str> {

    let term = term.to_lowercase();
    let mut results  = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&term){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = r"\ 
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = r"\ 
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."], search_case_insensitive(query, contents));
    }
}
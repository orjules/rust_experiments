use std::fs;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn two_results(){
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nAlso think about the productivity.";
        assert_eq!(vec!["safe, fast, productive.", "Also think about the productivity."], search(query, contents));
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    if config.query == "*" {
        println!("{}", contents);
        return Ok(());
    }
    for result in search(&config.query, &contents){
        println!("{}", result);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: & 'a str) -> Vec<& 'a str>{
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments. First is query, second is filename.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}
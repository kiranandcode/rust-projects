use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod tests;

pub struct Config {
    query: String,
    filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String], case_sensitive: bool) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config {query, filename, case_sensitive});
    }
}



pub fn run(config: Config) -> Result<(), Box<Error>>{

    let mut f = File::open(config.filename)?;



    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents) 
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    return Ok(());

}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}

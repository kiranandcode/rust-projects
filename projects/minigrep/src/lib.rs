use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod tests;

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Please provide exactly 2 arguments to this application.")
        }
    let query : String = args.get(1)
    .expect("Must supply exactly 2 arguments for minigrep.")
    .clone();

    let filename : String = args.get(2) 
    .expect("Must supply exactly 2 arguments for minigrep.")
    .clone();
        Ok(Config {query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("File not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;
    
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }


    Ok({})
}

pub fn search<'a>(query: &str, contents : &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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


    println!("With text:\n{}", contents);

    Ok({})
}
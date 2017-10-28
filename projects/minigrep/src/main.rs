extern crate minigrep;

use std::env;
use std::fs::File;
use std::process;
use std::io::prelude::*;
use std::error::Error;

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();;

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    let config: Config = Config::new(&args, case_sensitive).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
                 process::exit(1);
    });
            
   if let Err(e) = run(config) {
       println!("Application error: {}", e);
       process::exit(1);
   }
}




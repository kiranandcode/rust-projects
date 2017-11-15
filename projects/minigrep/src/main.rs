extern crate minigrep;
use std::env;
use std::process::exit;
use minigrep::Config;

fn main() {
   let args: Vec<String> = env::args().collect();
    let config : Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {}", e);
        exit(-1);
    }

}

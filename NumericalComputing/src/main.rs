extern crate regex;
use parsing::Scanner;
mod parsing;

fn main() {
    println!("Hello, world!");
    let mut parser = Scanner::new("3.1pipi+kiran-joe".to_owned());
    let mut i = 0;

    while let Some(tok) = parser.next() {
        println!("{:?}", tok);
    }
}

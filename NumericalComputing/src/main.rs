extern crate regex;
use parsing::Scanner;
mod parsing;
use parsing::ast::parse_expression;

fn main() {
    println!("Hello, world!");
    let mut parser = Scanner::new("3.1*(pi+10.0)".to_owned());
    let mut i = 0;

    if let Some(tok) = parse_expression(&mut parser) {
        println!("final output {:?}", tok);
    }
}

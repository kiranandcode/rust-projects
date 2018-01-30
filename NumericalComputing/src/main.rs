extern crate regex;
use parsing::Scanner;
mod parsing;
use parsing::ast::parse_expression;
mod expression;
use expression::Expr;
use expression::Formula;

fn main() {
    println!("Hello, world!");
    let mut parser = Scanner::new("x^2.0+x".to_owned());
    let mut i = 0;

    if let Some(tok) = Formula::from_scanner(parser) /*parse_expression(&mut parser)*/ {
        println!("final output {:?}", tok);
//        println!("Parsed: {:?}", Expr::from_ast_expression(tok));
    }
}

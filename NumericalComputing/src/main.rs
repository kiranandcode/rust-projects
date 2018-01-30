extern crate regex;
use parsing::Scanner;
mod parsing;
#[macro_use]
mod expression;

use parsing::ast::parse_expression;
use expression::Expr;
use expression::Formula;

fn main() {
    println!("Hello, world!");
    let mut parser = Scanner::new("x^2.0+x".to_owned());
    let mut i = 0;

    let valuation = valuation!("x" => 3.0);
    if let Some(tok) = Formula::from_scanner(parser) /*parse_expression(&mut parser)*/ {
        println!("final output {:?}", tok);
        println!("Evaluate: {:?}", tok.eval(&valuation));
//        println!("Parsed: {:?}", Expr::from_ast_expression(tok));
    }
}

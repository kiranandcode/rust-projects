extern crate regex;
mod secant;
mod parsing;
#[macro_use]
mod expression;

use parsing::ast::parse_expression;
use parsing::Scanner;
use expression::Expr;
use expression::Formula;
use secant::secant_find_root;


fn main() {
    println!("Hello, world!");
    let mut parser = Scanner::new("x^2.0+x-4.0".to_owned());
    let mut i = 0;

    if let Some(tok) = Formula::from_scanner(parser) /*parse_expression(&mut parser)*/ {
        println!("final output {:?}", tok);
        println!("Evaluate: {:?}", tok.eval(&valuation!("x" => 3.0)));
        println!("Derived: {:?}", tok.derive("x"));
        println!("Derived eval: {:?}", tok.derive("x").eval(&valuation!("x" => 3.0)));
//        println!("Parsed: {:?}", Expr::from_ast_expression(tok));
        let root = secant_find_root(&tok, -5.0, 0.1, 100);
        println!("Secant root: {}, {}", root, tok.single_variable_eval(root));
    }
}

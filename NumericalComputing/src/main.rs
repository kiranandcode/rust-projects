extern crate regex;
mod secant;
mod newton_raphson;
mod parsing;
#[macro_use]
mod expression;

use parsing::ast::parse_expression;
use parsing::Scanner;
use expression::Expr;
use expression::Formula;
use secant::secant_find_root;
use newton_raphson::newton_raphson_find_root;


fn main() {
    println!("Hello, world!");
    let mut parser = Scanner::new("((0.0-1.0)*150.0*x^4.0)+(1500.0*x^3.0)+(180.0*x^2.0)+(250.0*x)+100.0".to_owned());
    let mut i = 0;

    if let Some(tok) = Formula::from_scanner(parser) /*parse_expression(&mut parser)*/ {
        println!("final output {:?}", tok);
        println!("Evaluate: {:?}", tok.eval(&valuation!("x" => 3.0)));
        println!("Derived: {:?}", tok.derive("x"));
        println!("Derived eval: {:?}", tok.derive("x").eval(&valuation!("x" => 3.0)));
//        println!("Parsed: {:?}", Expr::from_ast_expression(tok));
        let root = secant_find_root(&tok, 1.0, 0.01, 100);
        let nr_root = newton_raphson_find_root(&tok, 10.0, 1000);
        println!("Secant root: {}, {}", root, tok.single_variable_eval(root));
        println!("newton root: {}, {}", nr_root, tok.single_variable_eval(nr_root));
    }
}

extern crate regex;
mod secant;
mod newton_raphson;
mod parsing;
#[macro_use]
mod expression;

use std::fs::File;
use std::io::{Write};
use parsing::ast::parse_expression;
use parsing::Scanner;
use expression::Expr;
use expression::Formula;
use secant::secant_find_root;
use secant::secant_find_root_manual;
use newton_raphson::newton_raphson_find_root;
use newton_raphson::newton_raphson_find_root_manual;


fn f(x : f64) -> f64 {
    // -285.0 * x.powf(6.0) 
    // + 41.0 * x.powf(5.0) 
    // + 74.0 * x.powf(4.0) 
    // + 123.0 * x.powf(3.0) 
    // + 90.0 * x.powf(2.0) 
    // + 55.0 * x.powf(1.0) 
    // - 11.0
    -96000.0 * x.powf(60.0) +
    1028.61 * (1.0 / (x - 1.0)) * x.powf(60.0) -
    1028.61 * (1.0 / (x-1.0)) + 97662.97
}

fn f_prime(x : f64) -> f64 {
    -285.0 * 6.0 * x.powf(5.0) 
    + 41.0 * 5.0 * x.powf(4.0) 
    + 74.0 * 4.0 *  x.powf(3.0) 
    + 123.0 * 3.0 * x.powf(2.0) 
    + 90.0 * 2.0 * x.powf(1.0) 
    + 55.0 
}



fn main() {
    // println!("Hello, world!");
    
//    let mut parser = Scanner::new("((0.0-1.0)*150.0*x^4.0)+(1500.0*x^3.0)+(180.0*x^2.0)+(250.0*x)+100.0".to_owned());
    // let mut parser = Scanner::new("((0.0 - 1.0) * 285.0 * x ^ 6.0) + (41.0 * x^5.0) + (74.0 * x ^ 4.0) + (123.0 * x ^ 3.0) + (90.0 * x ^ 2.0) + (55.0 * x) + ((0.0-1.0) * 11.0)".to_owned());
    // let mut i = 0;

    // if let Some(tok) = Formula::from_scanner(parser) /*parse_expression(&mut parser)*/ {
    //     println!("final output {:?}", tok);
    //     println!("Evaluate: {:?}", tok.eval(&valuation!("x" => 3.0)));
    //     println!("Derived: {:?}", tok.derive("x"));
    //     println!("Derived eval: {:?}", tok.derive("x").eval(&valuation!("x" => 3.0)));
//        println!("Parsed: {:?}", Expr::from_ast_expression(tok));
    //     let root = secant_find_root(&tok, 87.0 / 1127.0, 0.01, 100);
    //     let nr_root = newton_raphson_find_root(&tok, 87.0 / 1127.0, 1000);
    //     println!("Secant root: {}, {}", root, tok.single_variable_eval(root));
    //     println!("newton root: {}, {}", nr_root, tok.single_variable_eval(nr_root));
    // }
    let mut list = Vec::new();


    // let result = newton_raphson_find_root_manual(
    //     f,
    //     f_prime,
    //     1.0771,
    //     100000,
    //     &mut list,
    //     1
    // );
    // println!("newton root: {}, {}", result, f(result));

    let result = secant_find_root_manual(
        f,
        1.01092,
        0.001,
        100000,
        &mut list,
        1);
    // let result = 10.0;
    // let list = vec![0.0,0.1,0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.99, 0.9999, 1.01, 1.02, 1.03];
    // for val in list {
    //     println!("secant root: {}, {}", val, f(val));
    // }
    println!("secant root: {}, {}", result , f(result));
    println!("Iterations: {:?}", list);
    // let mut fil = File::create("output.csv").expect("Unable to create output");
    // writeln!(fil, "iteration, x, f(x)");
    // for i in &list {
    //     writeln!(fil, "{}, {}, {},", i.0, i.1, i.2);
    // }
    
}

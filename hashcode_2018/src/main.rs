extern crate hashcode_2018;
extern crate rand;

mod parser;
mod problem;
mod matrix;

use std::fs::File;
use std::io::{Write,self};
use problem::ride::Ride;
use problem::Problem;
use parser::read_input;

fn main() {
    println!("Hello, world!");
    let problem = read_input("b_should_be_easy");
    println!("problem is {:?}", problem);

    let solution = problem.solve();
    let mut output = File::create("output.txt").expect("Could not open output file");
    write!(output, "{}", solution);

}

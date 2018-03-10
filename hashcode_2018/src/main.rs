extern crate hashcode_2018;
extern crate rand;
extern crate chan_signal;

mod parser;
mod problem;
mod matrix;

use std::fs::File;
use std::io::{Write,self};
use problem::ride::Ride;
use problem::Problem;
use parser::read_input;

fn main() {
    let problem_name = "c_no_hurry";
    let problem = read_input(problem_name);
    println!("problem is {:?}", problem);

    let solution = problem.solve();
    let mut output = File::create(format!("output/{}.out", problem_name)).expect("Could not open output file");
    write!(output, "{}", solution);

}

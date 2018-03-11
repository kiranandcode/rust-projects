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
//    let (problem_name,low_expt_max, high_expt_max) = ("b_should_be_easy", 2100, 27000);
//   let (problem_name, low_expt_max, high_expt_max) = ("d_metropolis", 19000, 900000);
//    let (problem_name, low_expt_max, high_expt_max) = ("e_high_bonus", 4000, 900000);
//    let (problem_name, low_expt_max, high_expt_max) = ("c_no_hurry", 5000, 9000);
//    let (problem_name, low_expt_max, high_expt_max) = ("b_should_be_easy", 2100, 27000);



fn main() {
//    let (problem_name,low_expt_max, high_expt_max) = ("b_should_be_easy", 2100, 27000);
   let (problem_name, low_expt_max, high_expt_max) = ("d_metropolis", 1900, 9000);
//    let (problem_name, low_expt_max, high_expt_max) = ("e_high_bonus", 4000, 900000);
//    let (problem_name, low_expt_max, high_expt_max) = ("c_no_hurry", 5000, 9000);
//    let (problem_name, low_expt_max, high_expt_max) = ("b_should_be_easy", 2100, 27000);

    let problem = read_input(problem_name);
    println!("problem is {:?}", problem);
    let solution = problem.solve(low_expt_max, high_expt_max);
    let mut output = File::create(format!("output/{}.out", problem_name)).expect("Could not open output file");
    write!(output, "{}", solution);

}

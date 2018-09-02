use std::io;
use std::io::prelude::*;



pub fn run() {
    // basic boilerplate
    let input = io::stdin();
    let mut string = String::new();

    // read in the input
    input.read_line(&mut string).unwrap();

    let elems = string.split(" ").map(|seq| seq.trim().parse::<u64>().unwrap()).collect::<Vec<_>>();

    // sweet and sexy
    let (N,K) = (elems[0], elems[1]);

    let whole_number_count = (N / K) as u64;
    let mut values = whole_number_count * whole_number_count * whole_number_count; // choosing 3 from #whole_number count

    // now if we need to consider halves
    if K % 2 == 0 && K/2 > 0 {
        let half_and_whole_number_count = N / (K / 2);
        let half_number_count = half_and_whole_number_count - whole_number_count;
        values += half_number_count * half_number_count * half_number_count;
    }


    println!("{}", values)
}

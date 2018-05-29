use std::sync::Arc;
use std::thread;

fn main() {
    let numbers : Vec<_> = (0..100u32).collect();
    let mut joinHandles = Vec::new();


    for offset in 0..8 {
        let shared_numbers = numbers.to_owned(); 
        joinHandles.push(
            thread::spawn(
                move || {
                    let child_numbers = shared_numbers.to_owned();
                    let mut i = offset;
                    let mut sum = 0;
                    while i < child_numbers.len() {
                        sum += child_numbers[i];
                        i += 5;
                    }

                    println!("The sum of offset {} is {}", offset, sum);
                }
            ));
    }

    for handle in joinHandles.into_iter() {
        handle.join().unwrap();
    }
}

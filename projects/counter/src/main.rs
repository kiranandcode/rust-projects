
mod lib;
use lib::Counter;

///
/// main function for this binary
/// 
/// # Example
/// ```
/// 
///     main(); 
/// 
/// 
/// ```
///
///
///
fn main() {
    println!("Hello, world!");

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                    .map(|(a, b)| a * b)
                    .filter(|x| x % 3 == 0)
                    .sum();
    println!("{}", sum);

}

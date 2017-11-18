extern crate genetic_algorithm_cipher;
use genetic_algorithm_cipher::{
    get_corpus,
    make_request
};
fn main() {
   println!("{}", make_request().expect("Error while making a request"));
    println!("{}", get_corpus().unwrap());
    println!("Hello, world!");

}

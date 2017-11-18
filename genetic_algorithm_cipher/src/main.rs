extern crate genetic_algorithm_cipher;
use genetic_algorithm_cipher::{
    get_corpus,
    make_request
};
fn main() {
    println!("{}", get_corpus().unwrap());
}

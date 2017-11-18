extern crate genetic_algorithm_cipher;
use genetic_algorithm_cipher::corpus::{
    get_corpus
};
fn main() {
    println!("{}", get_corpus().unwrap());
}

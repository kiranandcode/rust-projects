extern crate genetic_algorithm_cipher;
use genetic_algorithm_cipher::corpus::{
    get_corpus
};
use genetic_algorithm_cipher::alphabet::{
    SubstitutionCipher
};
fn main() {
//    println!("{}", get_corpus().unwrap());
    let cipher : SubstitutionCipher = SubstitutionCipher::new();    
    println!("{}", cipher);
    
}

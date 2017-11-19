extern crate genetic_algorithm_cipher;
use genetic_algorithm_cipher::corpus::{
   get_corpus
};
use genetic_algorithm_cipher::alphabet::{
    SubstitutionCipher
};
fn main() {
//    println!("{}", get_corpus().unwrap());
    let cipherA: SubstitutionCipher = SubstitutionCipher::new();    
    let cipherB: SubstitutionCipher = SubstitutionCipher::new();    
    let cipherC: SubstitutionCipher = SubstitutionCipher::crossover(&cipherA, &cipherB);
    println!("{}", cipherA);
    println!("{}", cipherB);
    println!("{}", cipherC);
    let mut text = "kiran has written a simple substitution cipher!".to_string();

    println!("stringI: {}", text);

    println!("cipherA: {}", cipherA.apply(&text));
    println!("cipherB: {}", cipherB.apply(&text));
    println!("cipherC: {}", cipherC.apply(&text));
    
    println!("formatted corpus: {}", get_corpus().unwrap());
}

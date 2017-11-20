extern crate genetic_algorithm_cipher;
use genetic_algorithm_cipher::corpus::{
   get_corpus
};
use genetic_algorithm_cipher::alphabet::{
    SubstitutionCipher
};
use genetic_algorithm_cipher::ngram::{
    NgramFrequency
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

    let mut text_A = cipherA.apply(&text);
    println!("cipherA: {}", cipherA.apply(&text));
    println!("cipherB: {}", cipherB.apply(&text));
    println!("cipherC: {}", cipherC.apply(&text));
    
    let corpus = get_corpus().unwrap();
    println!("formatted corpus: {}", corpus);

    let frequency = NgramFrequency::generate_from(&corpus, 4);
    println!("frequency analysis: {}", frequency);
    println!("corpus score: {}", frequency.score_text(&corpus));
    println!("text score: {}", frequency.score_text(&text));
    println!("cipherA score: {}", frequency.score_text(&text_A));
}

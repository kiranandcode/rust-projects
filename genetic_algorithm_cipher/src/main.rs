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

use genetic_algorithm_cipher::GA::{
    Genome,
    Generator,
    Evaluator,
    EvolutionaryAlgorithmStrategy,
    GeneticAlgorithm
};

use genetic_algorithm_cipher::GA::strategy::HillClimbing::{ HillClimbingStrategy};
use genetic_algorithm_cipher::GA::strategy::SteepestAscentHillClimbing::{ SteepestAscentHillClimbingStrategy };

pub struct CipherGenome {
    cipher : SubstitutionCipher
}

impl Genome for CipherGenome {
    fn mutate(&mut self) {
        self.cipher.mutate();
    }
    fn crossover(parent_a : &Self, parent_b : &Self) -> Self{
        let cipher = SubstitutionCipher::crossover(&parent_a.cipher, &parent_b.cipher);
        CipherGenome {
            cipher
        }
    }
    fn clone(&self) -> Self {
       let cipher = self.cipher.clone();
       CipherGenome {
            cipher
       }
    }
}

pub struct CipherEvaluator {
    frequency_table : NgramFrequency,
    encrypted_text : String
}

impl Evaluator for CipherEvaluator {
    type Genome = CipherGenome;
    fn fitness(&self, genome : &Self::Genome) -> f64 {
       let converted_text = genome.cipher.apply(&self.encrypted_text);

       self.frequency_table.score_text(&converted_text)
    }
}

pub struct CipherGenerator {}

impl Generator for CipherGenerator {
    type Genome = CipherGenome;

    fn create_new(&self) -> Self::Genome {
        let cipher = SubstitutionCipher::new();

        CipherGenome {
            cipher 
        }
    }
}


fn main() {
//    println!("{}", get_corpus().unwrap());
//    let cipherA: SubstitutionCipher = SubstitutionCipher::new();    
//    let cipherB: SubstitutionCipher = SubstitutionCipher::new();    
//    let cipherC: SubstitutionCipher = SubstitutionCipher::crossover(&cipherA, &cipherB);
//    println!("{}", cipherA);
//    println!("{}", cipherB);
//    println!("{}", cipherC);
//    let mut text = "kiran has written a simple substitution cipher!".to_string();
//
//    println!("stringI: {}", text);
//
//    let mut text_A = cipherA.apply(&text);
//    println!("cipherA: {}", cipherA.apply(&text));
//    println!("cipherB: {}", cipherB.apply(&text));
//    println!("cipherC: {}", cipherC.apply(&text));
//    
//    let corpus = get_corpus().unwrap();
//    println!("formatted corpus: {}", corpus);
//
//    let frequency = NgramFrequency::generate_from(&corpus, 4);
//    println!("frequency analysis: {}", frequency);
//    println!("corpus score: {}", frequency.score_text(&corpus));
//    println!("text score: {}", frequency.score_text(&text));
//    println!("cipherA score: {}", frequency.score_text(&text_A));
    let corpus = get_corpus().unwrap(); 
    let frequency = NgramFrequency::generate_from(&corpus, 4);
    let mut text = "ljsboibtxsjuufobtjnqmftvctujuvujpodjqifs".to_string();
    let mut copy = text.clone();
    let encryption = SubstitutionCipher::new();
//    let alternate = encryption.apply(&corpus);
    
    let generator = CipherGenerator {};
    let evaluator = CipherEvaluator {
        frequency_table: frequency,
        encrypted_text: text 
    };

    let mut genetic_algorithm = GeneticAlgorithm::new(generator, evaluator);
    let mut hill_climbing = SteepestAscentHillClimbingStrategy::new(200, 10);
    let best = genetic_algorithm.execute_strategy(&mut hill_climbing);
    println!("Original: {}", copy);
    println!("Decrypted: {}", best.cipher.apply(&copy));

}

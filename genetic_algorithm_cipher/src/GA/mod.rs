pub mod interface;

pub use self::interface::{
    Genome,
    Generator,
    Evaluator
};

pub struct GeneticAlgorithm<T,G,E>
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T>
{
    generator : G,
    evaluator : E,
    population : Vec<T>
}


impl <T,G,E> GeneticAlgorithm<T,G,E> {
    pub fn new(population_size : usize) {

    }
}


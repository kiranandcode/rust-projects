pub mod interface;
pub mod strategy;

pub use self::interface::{
    Genome,
    Generator,
    Evaluator,
    EvolutionaryAlgorithmStrategy
};

pub struct GeneticAlgorithm<T,G,E>
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T>,
{
    generator : G,
    evaluator : E,
}


impl <T,G,E> GeneticAlgorithm<T,G,E> 
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T>
{
    pub fn new(generator : G, evaluator : E) -> GeneticAlgorithm<T,G,E> {
      GeneticAlgorithm {
           generator,
           evaluator,
       }
    }


    pub fn execute_strategy(&mut self, strategy : &mut EvolutionaryAlgorithmStrategy<Genome=T, Generator=G, Evaluator=E>) -> T {
       strategy.execute(&mut self.generator, &mut self.evaluator)
    }
}


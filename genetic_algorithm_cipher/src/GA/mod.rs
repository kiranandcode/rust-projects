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
    population : Vec<T>
}


impl <T,G,E> GeneticAlgorithm<T,G,E> 
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T>
{
    pub fn new(population_size : usize, generator : G, evaluator : E) -> GeneticAlgorithm<T,G,E> {
       assert!(population_size > 0);
       let mut items : Vec<T> = Vec::new();

       for i in 0..population_size {
           items.push(generator.create_new());
       }

       GeneticAlgorithm {
           generator,
           evaluator,
           population: items
       }
    }


    pub fn execute_strategy(&mut self, strategy : &mut EvolutionaryAlgorithmStrategy<Genome=T, Generator=G, Evaluator=E>) -> T {
       strategy.execute(&mut self.population, &mut self.generator, &mut self.evaluator)
    }
}


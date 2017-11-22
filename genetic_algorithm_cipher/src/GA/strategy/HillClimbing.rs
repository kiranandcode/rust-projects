use ::std::marker::PhantomData;
use GA::interface::{
    Genome,
    Generator,
    Evaluator,
    EvolutionaryAlgorithmStrategy
};


pub struct HillClimbingStrategy<T,G,E>
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T>
{
   genome : PhantomData <T>,
   generator : PhantomData <G>,
   evaluator : PhantomData <E>,
   iterations : u32
}

impl <T,G,E> HillClimbingStrategy<T,G,E> 
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T>
{
    pub fn new(iterations : u32) -> HillClimbingStrategy<T,G,E> {
        HillClimbingStrategy{
            genome : PhantomData,
            generator : PhantomData,
            evaluator : PhantomData,
            iterations
        }
    }
}

impl <T,G,E> EvolutionaryAlgorithmStrategy for HillClimbingStrategy<T,G,E>
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T> {
        type Genome = T;
        type Generator = G;
        type Evaluator = E;

        fn execute(&mut self, population : &mut Vec<Self::Genome>,  generator : &mut Self::Generator, evaluator : &mut Self::Evaluator) -> T {
            let mut s = generator.create_new();
            let mut best_fitness = evaluator.fitness(&s);
            for i in (0..self.iterations) {
                let r = generator.create_new();
                let fitness = evaluator.fitness(&r);
                if fitness > best_fitness {
                   s = r; 
                   best_fitness = fitness;
                }
            }

            s
        }
  }

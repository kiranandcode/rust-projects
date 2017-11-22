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
   evaluator : PhantomData <E>
}

impl <T,G,E> HillClimbingStrategy<T,G,E> 
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T>
{
    pub fn new() -> HillClimbingStrategy<T,G,E> {
        HillClimbingStrategy{
            genome : PhantomData,
            generator : PhantomData,
            evaluator : PhantomData
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
            generator.create_new()
        }
  }

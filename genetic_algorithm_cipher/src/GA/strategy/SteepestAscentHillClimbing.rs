use ::std::marker::PhantomData;
use GA::interface::{
    Genome,
    Generator,
    Evaluator,
    EvolutionaryAlgorithmStrategy
};


pub struct SteepestAscentHillClimbingStrategy<T,G,E>
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T>
{
   genome : PhantomData <T>,
   generator : PhantomData <G>,
   evaluator : PhantomData <E>,
   iterations : u32,
   samples : u32
}

impl <T,G,E> SteepestAscentHillClimbingStrategy<T,G,E> 
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T>
{
    pub fn new(iterations : u32, samples : u32) -> SteepestAscentHillClimbingStrategy<T,G,E> {
       SteepestAscentHillClimbingStrategy {
            genome : PhantomData,
            generator : PhantomData,
            evaluator : PhantomData,
            iterations,
            samples
        }
    }
}

impl <T,G,E> EvolutionaryAlgorithmStrategy for SteepestAscentHillClimbingStrategy<T,G,E>
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T> {
        type Genome = T;
        type Generator = G;
        type Evaluator = E;

        fn execute(&mut self,  generator : &mut Self::Generator, evaluator : &mut Self::Evaluator) -> T {
            let mut s = generator.create_new();
            let mut best_fitness = evaluator.fitness(&s);
            for i in (0..self.iterations) {
                let mut r = s.clone();
                r.mutate();
                let mut r_fitness = evaluator.fitness(&r);

                for j in (0..self.samples) {
                    let mut w = s.clone();
                    w.mutate();
                    let w_fitness = evaluator.fitness(&w);
                    if w_fitness > r_fitness {
                        r_fitness = w_fitness;
                        r = w;
                    }
                }

                if r_fitness > best_fitness {
                   s = r; 
                   best_fitness = r_fitness;
                }
            }

            s
        }
  }

use ::std::marker::PhantomData;
use GA::interface::{
    Genome,
    Generator,
    Evaluator,
    EvolutionaryAlgorithmStrategy
};


pub struct SteepestAscentReplacementHillClimbingStrategy<T,G,E>
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

impl <T,G,E> SteepestAscentReplacementHillClimbingStrategy<T,G,E> 
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T>
{
    pub fn new(iterations : u32, samples : u32) -> SteepestAscentReplacementHillClimbingStrategy<T,G,E> {
       SteepestAscentReplacementHillClimbingStrategy {
            genome : PhantomData,
            generator : PhantomData,
            evaluator : PhantomData,
            iterations,
            samples
        }
    }
}

impl <T,G,E> EvolutionaryAlgorithmStrategy for SteepestAscentReplacementHillClimbingStrategy<T,G,E>
    where T : Genome,
          G : Generator<Genome=T>,
          E : Evaluator<Genome=T> {
        type Genome = T;
        type Generator = G;
        type Evaluator = E;

        fn execute(&mut self,  generator : &mut Self::Generator, evaluator : &mut Self::Evaluator) -> T {
            let mut s = generator.create_new();
            let mut best = s.clone();

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
                s = r;

                if r_fitness > best_fitness {
                   best = s.clone(); 
                   best_fitness = r_fitness;
                }
            }

            best 
        }
  }


pub trait Genome {
    fn mutate(&mut self);
    fn crossover(parent_a : &Self, parent_b : &Self) -> Self;
    fn clone(&self) -> Self;
}

pub trait Generator {
    type Genome : Genome;
    fn create_new(&self) -> Self::Genome;
}

pub trait Evaluator {
    type Genome;
    fn fitness(&self, genome : &Self::Genome) -> f64;
}

pub trait EvolutionaryAlgorithmStrategy {
    type Genome : Genome;
    type Generator : Generator<Genome=Self::Genome>;
    type Evaluator : Evaluator<Genome=Self::Genome>;
    fn execute(&mut self, population : &mut Vec<Self::Genome>,  generator : &mut Self::Generator, evaluator : &mut Self::Evaluator) -> Self::Genome;
}

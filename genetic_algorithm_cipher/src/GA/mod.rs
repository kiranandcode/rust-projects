
pub trait GeneticAlgorithm {
    type Genome : Genome;
    type Evaluator : Evaluator<Genome=Self::Genome>;
    type Generator : Generator<Genome=Self::Genome>;

    fn new(generator : Self::Generator, evaluator : Self::Evaluator) -> Self;
}

pub trait Genome {
    fn mutate(&mut self);
    fn crossover(parent_a : &Self, parent_b : &Self);
}

pub trait Generator {
    type Genome : Genome;
    fn create_new(&self) -> Self::Genome;
}

pub trait Evaluator {
    type Genome;
    fn fitness(&self, genome : &Self::Genome) -> f64;
}

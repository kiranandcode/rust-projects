
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

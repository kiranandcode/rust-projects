# Generic Genetic Algorithm Library in Rust
## Fast, Safe and Beautiful

I've tried to make a general domain independant set of tools that I can incorporate into a variety of domains, to get Genetic Algorithms set up quickly.

I've split the main interface into two parts. A Genetic Algorithm defined by the generator and evaluator and genome, and
a independant Interface for strategies. This allows for using a variety of Genetic Algorithm Techniques, without having to 
change the code. Allowing for more experiments to be conducted.

I've implemented a couple of strategies as examples as well.

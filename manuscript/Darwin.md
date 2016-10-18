# Darwin
We now are ready to implement the core of the evolutionary algorithm. Our goal
is to run a simulation of an evolution for a number of generations. Before we
can tie the preceding chapters together, we still need to provide a few key
ingredients

1. _mutation_, a means to change a Program in the hopes to be better adopted to
   the environment.
2. _crossover_, a means for two Programs to have offspring, having parts of
   their parents DNA.

## Nadezhda

### mutate
We will mutate a program in the following way. For each part of the program, we
will decide if we are changing it or not. The `MutateDecision` is responsible
for making this decision.

```rust
pub trait MutateDecision {
    fn should_mutate(&mut self) -> bool;
}
```

We will provide a trait `Mutable` that accepts a `MutateDecision` in a `mutate`
function. It will return a mutate variant of `Self`, in our case a `Program`.

```rust
pub trait Mutatable {
    fn mutate(&self, decide: &mut MutateDecision) -> Self;
}
```

One way of implementing `Mutatable` for `Program` is shown below.

```rust
impl Mutatable for Program {
    fn mutate(&self, mut decide: &mut MutateDecision) -> Program {
        match *self {
            Program::Forward(ref contained_program) => {
                if decide.should_mutate() {
                    Program::Backward(Box::new(contained_program.mutate(decide)))
                } else {
                    Program::Forward(Box::new(contained_program.mutate(decide)))
                }
            },
            Program::Backward(ref contained_program) => {
                if decide.should_mutate() {
                    Program::Forward(Box::new(contained_program.mutate(decide)))
                } else {
                    Program::Backward(Box::new(contained_program.mutate(decide)))
                }
            },
            Program::Stop => Program::Stop,
        }
    }
}
```

### Crossover
Crossover is combining the DNA of two parent to produce offspring. We will
always use the two-for-two method. I.e. take two parents and produce two
offspring.

For the Nadezhda setting, the high level crossover algorithm is outlined below.
We have parents A and B.

1. Randomly chose a cut-points in programs A and B.
2. The two children are

a. Everything before cut-point of A followed by everything after cut-point of B.
b. Everything before cut-point of B followed by everything after cut-point of A.

### Evolve 
Evolution is the culmination of our hard work. This ties in all the ingredients
we have been exploring. The gist of evolution is

1. Generate a random population.
2. Until you are satisfied repeat.
3. Create a successor population.

A succession can be made in a number of ways, but variations do not seem to
change the overall effect. Below we list some characteristic steps.

1. Determine the fitness of the individuals in the generation.
2. Pass top individuals directly into the next generation.
3. Repeat until population is big enough

a. Cross two parents and add children to the population.
b. Mutate an individual and add to the population.
c. A combination of the above.

## Exercises
1. Implement a RandomDecision. I.e. implement the trait `MutateDecision` by
   performing a random decision.
2. Can the mutate function shown change the length of a `Program`?
3. Implement crossover for Nadezhda.

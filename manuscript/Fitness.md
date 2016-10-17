# Fitness

We would like to give a score to how well our programs perform. This can be done
by introducing a _fitness function_. The fitness function is arguably the most
important part of evolutionary programming. Without a good fitness function, you
likely will not get results you are hoping for.

## Nadezhda
For our cockroach a `Score` will be nothing more than an alias for an integer.

```rust
pub type Score = i32;
```

A `ScoreCard` is a trait that can be implemented to assign a score to a program
in an environment.

```rust
pub trait ScoreCard {
    fn score(&self, program: Program, environment: Environment) -> Score;
}
```

We could create a dummy struct `Config` and implement `ScoreCard` for. A score
would be given as the squared distance of the cockroach from the food location.

```rust
impl ScoreCard for Config {
    fn score (&self, program: Program, environment: Environment) -> Score {
        let result = program.evaluate_on(environment);
        let score = 
            (result.cockraoch_location - result.food_location)
            .abs()
            .pow(2);
            
        score
    }
}
```

## Exercises
1. Determine the program length of a Nadezhda program and include it in the score.
2. How would you extend the Nadezhda ScoreCard to include weight factors for the
   different parts of the score?

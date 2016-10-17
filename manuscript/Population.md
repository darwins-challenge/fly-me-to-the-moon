# Population
In order to keep track of the individual programs we define a population.

## Nadezhda
Little more is needed for our Nadezhda problem then a vector of `Program`s. We
wrap a `Vec<Program>` in our `Population`.

```rust
pub struct Population(pub Vec<Program>);
```

## Exercises
1. Implement a way to create a Nadezhda population.

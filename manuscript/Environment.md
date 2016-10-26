# Environment
The environment is what the individuals are inhabiting. Their actions determine
in part how the environment changes. We will describe each environment.

## Nadezhda
For our trusty cockroach the environment is basic. We only need to keep track of
the source of food, and the location of the cockroach. We model that by
integers.

```rust
pub struct Environment {
    /// The location of the cockroach
    pub cockroach_location: i32,
    /// The location of the pile of food
    pub food_location: i32,
}
```

## Exercises
2. Creating structs with many fields is cumbersome.
   An [fluent interface](https://en.wikipedia.org/wiki/Fluent_interface) could
   alleviate the nuisance. Create an fluent interface for the various
   environment.  

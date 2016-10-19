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

## Moonlander
The moonlander environment is a bit more involved. It needs to keep track of the
state of the lander, which is characterized by

1. *x*, horizontal position from the intended landing spot.
2. *y*, vertical position above the ground.
3. *vx*, horizontal speed.
4. *vy*, vertical speed.
5. *o*, orientation in radians, where zero is upright.
6. *w*, rotational speed.
7. *fuel*, the percentage of fuel left.

This and more is captured in the following structure

```rust
pub struct SensorData {
    pub x:  Number,
    pub y:  Number,
    pub vx: Number,
    pub vy: Number,
    pub o:  Number,
    pub w:  Number,
    pub fuel: Number,
}
```

## Exercises
1. What kind of environment is appropriate for AGC?
2. Creating structs with many fields is cumbersome.
   An [fluent interface](https://en.wikipedia.org/wiki/Fluent_interface) could
   alleviate the nuisance. Create an fluent interface for the various
   environment.  

# Evaluate
Our Abstract Syntax Trees change the environment by evaluating themselves. In
this chapter we see how that can be accomplished.

## Nadezhda
We want our cockroach to evaluate her brain and change the environment
accordingly. For this we introduce a trait `Evaluator`.

```rust
pub trait Evaluator {
    fn evaluate_on(&self, environment: Environment) -> Environment;
}
```

If we would implement `Evaluator` for `Program` we could use it like.

```rust
let program: Program = forward!(forward!(forward!(stop!())));
let start: Environment = Environment::new(5);

let finish: Environment = program.evaluate_on(start);
```

The implementation of `Evaluator` for `Program` directly reflects the structure
of AST. For each encountered `Forward` we increment the cockroach position, for
each encountered `Backward` we decrement the coackroach position and we stop
when encountered `Stop`.

```rust
impl Evaluator for Program {
    fn evaluate_on(&self, environment: Environment) -> Environment {
        match *self {
            Program::Forward(ref contained_program) => {
                let next_environment =
                Environment { cockroach_location : environment.cockroach_location + 1,
                  .. environment
                };
                contained_program.evaluate_on(next_environment)
            },
            Program::Backward(ref contained_program) => {
                let next_environment =
                Environment { cockroach_location : environment.cockroach_location - 1,
                  .. environment
                };
                contained_program.evaluate_on(next_environment)
            },
            Program::Stop => {
                Environment { .. environment }
            }
        }
    }
}
```

## Moonlander
The only real difference with Nadezhda is that the moonlander has an external
agent, i.e. the gravity of the moon, working on the lander. As such, after each
decision the moonlander program makes, the world changes.

Depending on the program, either the rotational speed is adjusted, a thruster is
fired or nothing is done. Accordingly the orientation, the horizontal and
vertical speeds are updated as well as the positions.

Although it is somewhat laborious it is also straight forward once we know the
relevant physics. 

## Exercises
1. Write an evaluator for AGC.

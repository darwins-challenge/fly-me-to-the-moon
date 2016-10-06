# Random Generation of AST 
When we create a population of individuals, we do not want to create every
individual by hand. We would rather generate them by some random process. This
chapter explains how this can be achieved.

In short generating an Abstract Syntax Tree comes down to picking an
alternative at each level of the grammar. This will become clear in examples.

## Nadezhda
The Nadezhda grammar is implemented assert

```rust
pub enum Program {
    Forward(Box<Program>),
    Backward(Box<Program>),
    Stop,
}
```

We would like to generate a program with the following line of code:

```rust
let program = rand::Random();
```

For this to work we need to implement `rand::Rand` for `Program`. The basic
strategy is to generate a number from 0 up to 3 and depending on the number
create one of the three alternatives.

```rust
impl rand::Rand for Program {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        let random_number = rng.gen_range(0, 3);
        match random_number {
            0 => {
                let nested_program: Program = Program::rand(rng);
                Program::Forward(Box::new(nested_program))
            },
            1 => {
                let nested_program: Program = Program::rand(rng);
                Program::Backward(Box::new(nested_program))
            },
            _ => Program::Stop,
        }
    }
}
```

Note that for the `Forward` and `Backward` alternative we need to also create a
nested program that will be wrapped.

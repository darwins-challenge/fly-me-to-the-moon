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

## Generating more complex grammars
In generating random programs one quickly encounters a problem. The problem
manifest itself in occasional stack overflows. In order to examine the problem
we are taking a closer look at the heart of the problem.

Take a look at the following grammar

```plain
Tree -> leaf
      | node Tree Tree
```

It expresses a tree structure. If one would write corresponding abstract syntax
trees and generators for this grammar, the same problem would occur. If there
are equal chances to generate a `leaf` or a `node` out of a `Tree`, we are very
likely to end up with a very deep tree.

The solution is to bias the generation in favor of `leaf`s. This will short
circuit the infinite descent and makes stack overflows unlikely.

Mathematically inclined readers can follow along to investigate the issue
further. Lets assume that there is a chance {$$}p{/$$} to generate a `node` and a
chance {$$}q = 1 - p{/$$} to generate a `leaf`. We would like to express the
expected number of nodes and leafs {$$}E{/$$} in our generated trees. By
linearity of expected values we have

{$$}
E = p \cdot (1 + 2E) + q \cdot 1
{/$$}

Because for each `node` there is 1 extra node plus for the left and right
branches two times the expected number of nodes and leafs, and for each `leaf`
there is one extra leaf. Using the relation {$$}q = 1 - p{/$$} and solving for
{$$}E{$$} we get

{$$}
E = \frac{1}{1 - 2p} = \frac{1}{2q - 1}
{/$$}

If there are is an equal change to generate `nodes` and `leafs` this expression
diverges. I.e. the expected number of nodes and leafs will be infinite.

## Exercises
1. What is the expected length of a randomly generated Nadezhda program.
2. When writing generators it gets tedious to constantly write the same
   boilerplate code. Alleviate this nuisance by writing a macro.

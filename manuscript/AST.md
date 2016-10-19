# Abstract Syntax Trees
An [abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
(AST)

> is a tree representation of the abstract syntactic structure of source code
> written in a programming language 

For us it will just be an instance of a Rust data structure, that corresponds
with the grammar we want to represent.

## Nadezhda AST as enums
Rust has [enums](https://doc.rust-lang.org/book/enums.html) that are well suited
for the task of modeling ASTs. Once you have a grammar, it is a small step to
implement in Rust.

You will encounter one small problem. We will illustrate that with an example.
Take the Nadezhda grammar

```plain
Program -> forward Program
         | backward Program
         | stop
```

If we would translate this into Rust without really thinking we would come up
with something like the following code

```rust
enum Program {
    Forward(Program),
    Backward(Program),
    Stop,
}
```

Which is almost a direct translation. Unfortunately this does not compile. The
rust compiler spits out the following warning.

```plain
error: recursive type `Program` has infinite size [E0072]
enum Program {
    Forward(Program),
    Backward(Program),
    Stop,
}
help: run `rustc --explain E0072` to see a detailed explanation
help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `Program` representable
```

We remedy this by wrapping the inner programs in a `Box`.

```rust
pub enum Program {
    Forward(Box<Program>),
    Backward(Box<Program>),
    Stop,
}
```

## Moonlander
The translation of the Nadezhda grammar teaches us everything there is to know
for us to translate the moonlander grammar into abstract syntax trees. Taking a
look at the source, there a little, if any, surprises.

## Exercises
1. What would be other options to appease the Rust compiler when building
   recursive types? How would that differ from using `Box`.
2. Is `stop` necessary in the Nadezhda grammar?
3. Expression the AGC grammar as an abstract syntax tree.

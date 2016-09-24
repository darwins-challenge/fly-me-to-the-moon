# Parsers
[Parsers](https://en.wikipedia.org/wiki/Parsing) are a

> software component that takes input data (frequently text) and builds a data
> structure – often some kind of parse tree, abstract syntax tree or other
> hierarchical structure – giving a structural representation of the input,
> checking for correct syntax in the process 

This wordy definition is hard to take in. For us, a parser will be a means to go
from a syntax, i.e. characters typed in a file, to an Abstract Syntax Tree
(AST).

Creating parsers is a rich subject with very interesting nuances. We can largely
avoid creating our own parser by leveraging the excellent parser of Rust.

## Domain Specific Language
A
[Domain Specific Language](https://en.wikipedia.org/wiki/Domain-specific_language) (DSL)
is a 

> is a computer language specialized to a particular application domain.

We will make use of a _internal DSL_, i.e. a DSL embedded in Rust. This will
save us work, because we are leveraging the Rust parser.

We will achieve this by the use
of [macros](https://doc.rust-lang.org/book/macros.html). 

For an excellent treatment on DSL
consult [Domain Specific Languages](http://martinfowler.com/books/dsl.html).

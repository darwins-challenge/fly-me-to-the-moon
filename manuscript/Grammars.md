# Grammars
Grammars are a way to specify which structures are possible. There are a lot of
formal definitions of grammars. We on the other hand will go for a working
knowledge. I.e. we will describe how to specify a grammar, and how to implement
it in rust.

## Grammar Game
We will view grammars as a kind of game. Like any game the grammar game has some
objects to manipulate. The objects to manipulate will be sequences of symbols.
There will be two types of symbols: *terminal symbols* and *non-terminal
symbols*. The reason why the symbols are called that way will be given shortly.
Terminal symbols will be written with lowercase words. E.g `a`, `by` or `cat`.
Non-terminal symbols on the other hand will be written starting with an
uppercase letter, followed by lowercase letters.
E.g `A`, `By`, `Cat`.

Besides symbols also have *production rules*. A production rule is written as
follows

```plain
A -> aAb
```

It has a single non-terminal symbol on the left of an arrow. To the right of the
arrow is a sequence of symbols. A grammar game can have multiple production
rules, even with the same non-terminal symbol to the left of an arrow. If that
is the case, one is allowed to list the alternative sequences of symbols to the
right of the arrow separated by a pipe `|`. E.g. given a grammar with two
production rules `A -> aA` and `A -> b`, we are allowed to write this as:

```plain
A -> aA
   | b
```

There is an other convention that we will use. Sometimes we will want to replace
a non-terminal symbol with the empty string. This will be represented by a
production rule like `A -> ''`.

Below you find an example of a grammar that will play a role in the exercises.

```plain
A -> aAb
   | B
B -> c 
   | ''
```

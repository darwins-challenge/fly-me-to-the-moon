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

### How to play?
We are discussing the grammar game, but we failed to mention how to play this
game. The game is played by starting with a non terminal symbol. That will be
are current sequence of symbols. Next we chose a non terminal symbol from our
current sequence, find a production rule which has that symbol to the left of an
arrow and _apply_ it. We apply it by replacing the occurrence of the left side
of the arrow by the sequence of symbols to the right. This will give us a new
current sequence and we can choose a new production rule. We continue as long as
there are non-terminal symbols to choose from our current sequence of symbols.

Let's work out an example. Take the grammar above. We start with the
non-terminal symbol `A`. We choose the production rule `A -> aAb`. Applying it
results in the follow sequence of symbols `aAb`. We choose the same production
rule again and apply it again, resulting in `aaAbb`. Next we choose the
alternative production rule for `A`. I.e. `A -> B`, and apply it to `aaAbb`.
This transforms into `aaBbb`. As a last application we choose the production
rule `B -> ''` resulting in `aabb`, and we are finished.

We call the act of applying production rules *deriving*, and when we come to a
sequence of only terminal symbols, we say that this sequence can be derived from
the non-terminal symbol we started with.

## Nadezhda
For the Nadezhda project we are going to look into the following grammar.

```plain
Program -> forward Program
  -> backward Program
  -> stop
```

This grammar models the decisions our robot cockroach will make in finding her
virtual food supply.

## Exercises
1. Which of the following sequences can be derived from the non-terminal symbol
   `A` given the grammar
   
   ```plain
   A -> aAb
      | B
   B -> c
      | ''
   ```
   
   * ''
   * c
   * cc
   * ab
   * abb
   * aab
   * aabb
   * aacbb
   * aaccbb
2. For the Nadezhda grammar, is there a limit on the number of forward symbols
   in a sequence that is derived?
3. Create a grammar that is capable of expressing calculations.

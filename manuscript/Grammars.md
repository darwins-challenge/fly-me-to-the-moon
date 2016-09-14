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

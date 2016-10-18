# What this book is all about
This book is an compendium to a workshop exploring
[genetic programming][genetic-programming]. Genetic programming is

> a technique whereby computer programs are encoded as a set of genes that are
> then modified (evolved) using an evolutionary algorithm.

This book will explain various ideas and will ground them by discussing ways you
could implement them. It will provide you with a practical introduction to
genetic programming and hones it with exercises, both pen and paper as
implementation exercises.

Each chapter will introduce ideas needed to create your own genetic programming
environment, that can solve a certain problem. The ideas will be illustrated by
three projects.

1. [Nadezhda][cockroach] was the name of a cockroach sent into space to study
   its behavior. Even though cockroaches have far more intricate behavior than
   we attribute them during this problem, we like to stay in the space theme.
   You can inspect the code in the `nadezhda` directory.  
2. [AGC][agc] is an abbreviation for _Apollo's Guidance Computer_ that was
   responsible for aiding in computations of guidance, navigation and control
   of the space craft. If you want, you could implement this yourself, if you
   follow the exercises in this book.
3. [Lunar Lander][lunar] tries to land safely on the surface of the moon. The
   catch is that you are giving control to a program that a computer wrote. This
   project demonstrates that hard problems can be solved by genetic programming.
   You can inspect the result in `moonlander-ast-rust`.

[genetic-programming]: https://en.wikipedia.org/wiki/Genetic_programming
[cockroach]: https://en.wikipedia.org/wiki/Nadezhda_(cockroach)
[agc]: https://en.wikipedia.org/wiki/Apollo_Guidance_Computer
[lunar]: http://moonlander.seb.ly/

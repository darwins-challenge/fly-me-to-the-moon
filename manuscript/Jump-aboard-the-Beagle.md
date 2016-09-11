# Jump aboard the HMS Beagle[^beagle]
In this chapter we will give a lay-mans introduction to evolution and how it
applies to the projects we are studying.

## Evolution
Evolution tracks a *population* over time in their *environment*. A population
consists of *individuals*. Individuals differ by there *genetic makeup*.
The survival of the genetic makeup of a individuals is determined by their
*fitness*, i.e. how well they are adapted to their environment. Individuals that
are fitter, have a higher chance of reproducing, allowing their genetic makeup
to prosper.

## Translation
Above we gave in broad strokes a general description of evolution. In this
section we are going to paint a little bit more detail. We do this by providing
a translation, from the general term to the specific term that we are going to
use.

### Individuals
Our individuals will be _abstract syntax trees_ or _AST_ for short. Abstract
syntax trees are a means to accurately describe the structure of a certain
program. We will use grammars to define which abstract syntax trees are allowed.

### Genetic Makeup
The behavior of a program is determined by evaluating an AST. So the genetic
makeup is the AST itself.

### Environment
We have a number of environments that our AST will flourish in. The environments
are described in the About chapter.

### Fitness
The fitness of our AST will be measured by a fitness function. This is a
function that tracks the performance of a AST in her environment and assigns a
numerical value to it. This numerical value will be used to compare AST to
determine their relative fitness. 

## Further reading
Below we have some suggestions on what to read next.

If you are interested int grammars, abstract syntax trees, and parsers, read the
corresponding chapters.

Curious about how to create random individuals go to the Alea Iacta Est chapter.

To learn about the environment and how to simulate it head over to the
environment chapter.

[^beagle]: The [HMS Beagle](https://en.wikipedia.org/wiki/HMS_Beagle) was the
    ship that took
    [Charles Darwin](https://en.wikipedia.org/wiki/Charles_Darwin) around the
    world. It wad during this trip that Darwin collected data that led him to
    formulate the idea of evolution. His work culminated into his seminal book
    [On the Origins of Species](https://en.wikipedia.org/wiki/On_the_Origin_of_Species).

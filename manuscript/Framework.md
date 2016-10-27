# A framework for Genetic Programming

Up until now, we've done everything by hand. You might notice that some of the
work is applicable, such as generating a random population of trees, and
applying reproduction, crossover and mutation to build new generations is
to every genetic programming task.

To help you solve genetic programs quickly, we've wrapped up most of the
machinations necessary to abstract these concerns away into a library. We're
going to introduce that library by taking you through the implementation of
a GP model for the Moonlander problem, that uses this library.

Both the generic library and the model to solve the moonlander problem are
available on GitHub. They're called `moonlander-gp` and `moonlander-ast`,
respectively, and the locations can be found in the resources.

## Overview of the framework

The operations that are provided by the framework are:

* Generating a new population with random trees
* Evolving the population using the three genetic operations:
    * Reproduction
    * Mutation
    * Crossover

To use the framework, you'll need to supply two things of your own:

* A set of AST node types, thereby specifying a grammar for the solution
* A fitness function to score solutions.

## Framework Traits

To work with ASTs in a generic way, we've had to implement quite some traits in
Rust. We'll quickly take you through them, before explaining that we won't need to
implement most of them yourself.

An important constraint for many of these traits is that concrete type
information cannot be used, because we'll pick random nodes at runtime, so
the types can not be known statically. Hence, some traits, like `Copyable`, will
duplicate similar traits like `Clone` that work on static information.

### AstNode

This trait should be implemented by all types that represent a node in the
abstract syntax tree. It is used to generically iterate over and construct
trees.

Because `Any::get_type_id()` is not stable, it also contains a method to return
an identifier per node type, so that we can match nodes of the same type in
different trees during crossover.

```rust
pub trait AstNode: Any+Mutatable+Copyable+Sync {
    fn node_type(&self) -> usize;
    fn children(&self) -> Vec<&AstNode>;
    fn replace_child(&self, old_child: &AstNode, new_child: &mut Option<Box<AstNode>>) -> Box<AstNode>;
}
```

### RandNode

This trait should be implemented to construct a random subtree with the root of
the given type. It is like `rand::Rand`, except the generic `Rng` argument has
been replaced with a reference to a generic `Rng` argument, because we'll need
to be calling this from trait functions on `AstNode`, and trait functions cannot
be generic.

For the implementation of this trait, you can use the `pick![]` macro.

The function also takes an argument of type `NodeWeights`, which is used to
control the height of trees that get generated. Whenever there's a choice to be
made between internal and leaf nodes in the tree, the weights on that object
will be used to control the relative probabilities. This (roughly) controls the
height of the generated subtree.

```rust
pub trait RandNode: Sized {
    fn rand(weights: NodeWeights, rng: &mut Rng) -> Self;
}
```

### Mutatable

This trait is used for mutation of a node, or a subtree. There is a default
implementation for all nodes that uses the `RandNode` implementation to replace
the subtree with a new random subtree, but you can also mutate nodes in a more
controlled fashion, such as replacing the operator in a node that represents a
binary operation, while keeping the operands the same, by implementing this
trait directly.

```rust
pub trait Mutatable {
    /// Return a mutation of this node
    fn mutate(&self, max_height: i32, rng: &mut Rng) -> Box<AstNode>;
}
```


### Copyable

As aforementioned, this is a trait that we use to construct a copy of a subtree.
Implementation of this trait comes for free for every type that implements
`Clone`.

```rust
pub trait Copyable {
    fn copy(&self) -> Box<AstNode>;
}
```

### impl_astnode!

Fortunately, you can get implementation of most of these traits for free for
enums by using the `impl_astnode!` macro, which implements both `AstNode` and
`RandNode`. This brings `Mutatable` for free, and if you `#[derive(Clone)]` as
well you get `Copyable` for free as well.

Hence, AST nodes and all of their traits can be defined as succinctly as:

```rust
#[derive(Clone)]
pub enum Condition {
	True,
	False,
	And(Box<Condition>, Box<Condition>),
    ...
	Less(Box<Expression>, Box<Expression>),
	LessEqual(Box<Expression>, Box<Expression>),
    ...
}

impl_astnode!(Condition, 0,
              leaf True(),
              leaf False(),
              int And(left, right),
              int Less(left, right),
              int LessEqual(left, right),
```

This defines an enum and some cases. The `impl_astnode!` macro takes some
the enum name, a node type number, and the various case arms, describing whether
they're internal or leaf nodes, and generates the required traits.

Mark internal nodes with the prefix `int`, and leaf nodes with the prefix
`leaf`. This will be used in the generated implementation of `RandNode` to
pick either the provided weight for internal nodes or for leaf nodes.

## Moonlander

In the moonlander problem, the objective is to evolve a program that will
succesfully control a spacecraft to safely land on the surface of a moon.

First, this requires that we have a spacecraft to control, a surface to land on,
and physics to which the lander will be subject.

### Simulation

As part of the evaluation of a program (i.e., calculating its fitness), we will
run the program through a simulation of the landing. This simulation will have a
model of physics. For instance, the moonlander will have a position `(x, y)` in space,
somewhere above the surface. It will have a velocity `(vx, vy)`, and every
_tick_ of the simulation, the position will be updated with the velocity. in
the same way, the ship will have a rotation and a rotational velocity.

The moonlander can affect its velocities, and hence ultimately its position, by
controlling the actuators on the ship. There are three actuators: the main
thrusters, boosting the ship forward in its current direction, or attitute
thrusters which will change the ships rotational velocity.

When the `y` position of the moonlander has hit `0`, signifying that it has hit
the surface, we'll evaluate the ship's angle and vertical velocity, and deem it
to having succeeded when the velocity is low enough and the angle is close
enough to vertical.

To make sure the program we're about to evaluate has enough information to make
correct decisions, we'll equip the ship with virtual "sensors", allowing it to
sense various properties about itself. In this case, we'll allow the ship access
to all properties of the simulation, so that includes the aforementioned `(x,
y)`, `(vx, vy)` and so on.

You can imagine a simulation in which there is information that the control
program does not have access to. For example, the exact velocity might not be
known. In this case however, we want to give our control program the greatest
chance of success, and we'll supply all information.

The information will be made available to the control program in the
`SensorData` structure:

```rust
pub struct SensorData {
    pub x:  Number,
    pub y:  Number,
    pub vx: Number,
    pub vy: Number,
    pub o:  Number,
    pub w:  Number,
    pub fuel: Number,
}
```

### Grammar

All computer programs have an associated grammar that describes the space of
possible programs. It's important that this space can _in principle_ contain
program that solve the problem.

There is a choice of the style of program to allow. We can consider generating
either imperative programs or functional programs. The space of functional
programs without recursion is very safe, because programs are guaranteed to
terminate, and there are no data dependencies between statements (such as
variables having been declared or initialized).

For this problem we'll opt to model a simple function that every tick takes the
`SensorData` and outputs a single `Command`, which is the action to perform that
round.

We'll start off even simpler than the fully general program, and simply generate
a program that evaluates to a boolean. We'll couple this boolean to the main
thrusters, firing the thrusters if the program returns `true`, and do nother
otherwise.

Obviously, this simple program will not allow us to do attitute control, because
there's no way for us to trigger the `Left` and `Right` thrusters, but we can
always extend the grammar later.

Let's start by defining the sensors:

We want to define arbitrary conditions. Those conditions will eventually contain
numerical expressions, and some of those numbers may come from the sensors.

This leads us to the following initial grammar:

Condition:

```rust
#[derive(Clone)]
pub enum Condition {
	True, False,

	Not(Box<Condition>),
    Or(Box<Condition>, Box<Condition>), And(Box<Condition>, Box<Condition>),

	Less(Box<Expression>, Box<Expression>), LessEqual(Box<Expression>, Box<Expression>),
	Equal(Box<Expression>, Box<Expression>),
    GreaterEqual(Box<Expression>, Box<Expression>), Greater(Box<Expression>, Box<Expression>),
}

impl_astnode!(Condition, 2,
              leaf True(), leaf False(),
              int Not(cond),
              int Or(left, right), int And(left, right),
              int Less(left, right), int LessEqual(left, right),
              int Equal(left, right),
              int GreaterEqual(left, right), int Greater(left, right));
```

Expression:

```rust
#[derive(Clone)]
pub enum Expression {
	  Constant(Number),
	  Sensor(Box<Sensor>),
	  Plus(Box<Expression>, Box<Expression>), Minus(Box<Expression>, Box<Expression>),
	  Multiply(Box<Expression>, Box<Expression>), Divide(Box<Expression>, Box<Expression>),
}

impl_astnode!(Expression, 3,
              leaf Constant((data n random_constant)),
              leaf Sensor(sensor),
              int Plus(left, right), int Minus(left, right),
              int Multiply(left, right), int Divide(left, right));
```

Sensor:

```rust
#[derive(Clone)]
pub enum Sensor { X, Y, Vx, Vy, O, W, Fuel, }

impl_astnode!(Sensor, 0,
              leaf X(), leaf Y(), leaf Vx(), leaf Vy(),
              leaf O(), leaf W(), leaf Fuel());
```

### Evaluation

In every step of the simulation, the program will be given a chance to decide
whether or not to fire its thrusters. This means we have to _evaluate_ the
AST of the program with the current values of the sensors to produce the output
for this decision.

In the moonlander framework, this means implementing the trait
`EvaluateToCommand` for the root node of the program. Evaluation of the AST uses
the following traits, whose implementations are quite straightforward (refer
to the `moonlander-ast` source if you're curious):

```rust
trait EvaluateToCommand {
	fn evaluate(&self, sensor_data: &SensorData) -> Command;
}

trait BooleanValue {
	fn is_true(&self, sensor_data: &SensorData) -> bool;
}

trait NumericValue {
	fn num_value(&self, sensor_data: &SensorData) -> Number;
}
```

The magic that connects the abstract and generic AST to the actual simulation is in the
implementation of `NumericValue` for `Sensor`:

```rust
impl NumericValue for Sensor {
    fn num_value(&self, sensor_data: &SensorData) -> Number {
        match *self {
            Sensor::X    => sensor_data.x,
            Sensor::Y    => sensor_data.y,
            Sensor::Vx   => sensor_data.vx,
            // ...etc...
        }
    }
}
```

### Fitness

Now that we can run the simulation, and can have the program make decisions
inside it that affect the simulation, it's time to define the criteria that make
a program successful.

We do this by writing a fitness function. This function assigns a numerical
score to the performance of a program in the simulation. The goal of the genetic
algorithm will be to maximize the value of the fitness function. It's therefore
helpful if higher values for the fitness function corresponds to better actual
performance in ways that we care about!

It's also helpful if the fitness function can be made as monotonically
increasing towards the better program space as possible. If the fitness function
returns 1,000 if the program finally met the victory condition, but 0 everywhere
else, the intermediate values will not help guide the evolutionary algorithm
towards the more successful part of the program space. The evolutionary
algorithm will most likely be wandering around arbitrary parts of the program
space, generating and selecting all equivalently successful programs (all
scoring 0), until by chance it _might_ hit upon a winning program. Depending on
how rare successful program are, this might take quite a while.

In the framework, the fitness function takes a reference to a program, and
returns any object that implements the `Fitness` trait. The only notable quality
of the `Fitness` trait is that it has something called a `ScoreCard`:

```rust
pub trait Fitness: Send {
    fn score_card(&self) -> &ScoreCard;
}
```

The `ScoreCard` represents the actual numerical score of an algorithm, which can
be composed of many subscores and penalties. This labeled subdivision will be
useful for human operators, to judge whether the scores assigned by the fitness
function make sense.

`Fitness` is a trait, so that any actual object may be substituted. This allows
us to collect and export extra data during simulation in a struct of our
choosing. In the `moonlander-ast` project, we use this to collect trace of
all frames of the moonlander during each simulation, so that we can export
and visualize those traces later.

We collect the fitness like this:

```rust
pub fn score_lander<P>(program: &P, _: &mut Rng, mut sensor_data: SensorData, world: &World) -> LandingTrace
    where P: EvaluateToCommand+AstNode
{
    // ...
    while !sensor_data.hit_ground {
        // ...
    }

    LandingTrace {
        trace: trace,
        score_card: ScoreCard::new(vec![
            ("survival_bonus",   3.0 * frames),
            ("fuel_bonus",       (100.0 * total_fuel / frames)),
            // ...more scores here...
        ])
    }
}
```

Of course, this gives us the score for one run.

We probably want to run the simulation multiple times under randomized
conditions. This will help us avoid overfitting the program to a fixed
problem, and generate programs that generalize well.

However, randomized conditions introduce the chance that a potentially good
program has a bad luck of the draw, performs poorly in its simulation, and gets
culled from the population.

To reduce the chances of this happening, we'll run the simulation multiple
times and average the scores. For that we use the function `score_lander_multi`,
which takes a number of rounds to evaluate, calls `score_lander` _N_ times,
and combines the results.

```rust
pub fn score_lander_multi<P, G>(n: usize, program: &P, rng: &mut Rng, sensor_data_fn: G, world: &World) -> LandingTrace
    where P : EvaluateToCommand+AstNode,
          G : Fn(&mut Rng) -> SensorData
```

# Tweak Fitness Function
The most important part of a genetic program is the fitness function. It guides
programs along a favorable evolutionary path if it is constructed properly, or
will let the programs run aground before the are well on their way.

We have created a few scenarios that of increasing complexity to test your
understanding of fitness functions.

## Straight Landing

### First scenario
The lander starts at a fixed height, without any rotation, and needs to
successfully land. To solve this scenario, it suffices to evaluate a `Condition`
program.

If the `Condition` evaluates to `true`, the lander will use its thrusters (the
command will be `Thrust`). If it doesn't, it won't (the command will be `Skip`).

### Second scenario
The previous scenario evolved a program that started at a fixed position.
However, such a winning program might be overfitting to the problem. In this
scenario, the lander starts at a random height.

Does your model still evolve a successful solution?

## With a Twist

### First scenario
In the preceding project, the lander always started upright. In this
project, it will start at angle.

Using the `Condition`, we could evaluate one of two commands: `Thrust` or
`Skip`. Once the lander also needs to correct its attitude, those two
commands are no longer sufficient (check: can `evolve_condition` evolve
a winning solution to this scenario?)

Instead, we'll need a new type of AST node, to increase the range of
programs that we can express.

Can you invent and implement such an AST node? (Don't forget to make a new
example to evolve it, and don't forget to update the fitness function)


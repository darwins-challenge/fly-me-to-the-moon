# What to Do
Here we give some suggestions for projects you could do

1. Straight Landing
2. With a Twist
3. AGC
4. Bring your Own

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

## AGC
We will try to fit the series of points with a function. You find a skeleton
project in the `AGC` directory. See the AGC chapter in the accompanying book for
a description of the book.

## Bring your own 
We would love to see this tool used to solve a problem that you have. There is a
skeleton project that will give you a running start. Other than that you are on
your own. 



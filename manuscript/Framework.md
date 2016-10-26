# Framework

## Moonlander
The moonlander grammar is fairly involved. It is responsible for expression
control logic for the lander. As such it is capable of making _decisions_ based
on _calculations_ involving _sensor readings_.

Listing the entire grammar would take up a lot of space. It is best to inspect
the grammar from the code.


## Moonlander
The moonlander environment is a bit more involved. It needs to keep track of the
state of the lander, which is characterized by

1. *x*, horizontal position from the intended landing spot.
2. *y*, vertical position above the ground.
3. *vx*, horizontal speed.
4. *vy*, vertical speed.
5. *o*, orientation in radians, where zero is upright.
6. *w*, rotational speed.
7. *fuel*, the percentage of fuel left.

This and more is captured in the following structure

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



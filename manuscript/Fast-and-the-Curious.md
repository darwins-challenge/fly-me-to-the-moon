# Appendix: Fast and the Curious
So you want to get to the result fast? You came to the right place. First things
first, you need to get a copy of the resources. You can find pointers on how to
obtain them from the Resources chapter.

Change directory to the `help` directory and use cargo to build
the `evolve_condition` example.

```shell
cargo run --release --example evolve_condition 1_fixed_vertical_landing.toml > fast-furious.json
```

Wait for the run to start and head over to the `moonlander-visualisation`
directory. Run that project providing it with a directory which has the
`fast-furious.json` file.

```shell
cargo run --release ../moonlander-ast/
```

Follow the programs instruction to open `localhost:8080` in a browser. To view a
trace of a run of evolution, open the load trace tab. Search for the
`fast-furious.json` file, hit the `load` button. A list of generations can be
seen. Click on one of them to see the performance of the moonlander. To see
later generations reload the `fast-furious.json` file.

If you want to stop, don not forget to quit the `evolve_condition` example program.

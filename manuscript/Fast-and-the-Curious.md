# Appendix: Fast and the Curious
So you want to get to the result fast? You came to the right place. First things
first, you need to get a copy of the resources. You can find pointers on how to
obtain them from the Resources chapter.

Change directory to the `moonlander-ast-rust` directory and use cargo to build
the `example` example.

```shell
cargo build --release --example evolve
```

Wait for the build to finish and head over to the `moonlander-visualisation`
directory. Run that project providing it with the freshly build `evolve` binary.

```shell
cargo run --release ../moonlander-ast-rust/target/release/examples/evolve
```

Follow the programs instruction to open `localhost:8080` in a browser. To start
a run of evolution hit the `start evolution` button. Each time an program
outperforms the previous best, an new entry in the generations lists is added.
Click a generation to see how one of the best programs performs.

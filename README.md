# push_swap_rpv

A `push_swap` tester for school 42, using the power of Rust and multi-threading to speed up your corrections!

## Usage
Compile the project with:
```sh
cargo build --release
```
Then the executable should be `target/release/push_swap_rpv`.

Alternatively, you can use:
```sh
cargo run --release -- 
```
And input all of your arguments after the `--`.

To see the help page:
```sh
cargo run --release -- --help
```

To launch it with 100 tries on 500 numbers:
```sh
cargo run --release -- --path [path_to_your_push_swap]
```

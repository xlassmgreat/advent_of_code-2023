# advent_of_code-2023
My solutions to Advent of Code 2023 in Rust. All solutions are in the bin folder. All binaries take their input from stdin.

A demo run might look like this:

```
cat inputs/day3 | cargo run --release --bin day3
```

For sake of ease, two scripts are provided.

1. ***run.sh***: takes one integer as argument, calls `cargo run` for the solution of the corresponding day, passing the rest of the arguments to cargo as well.
2. ***solve.sh***: Like `run.sh`, but also passes the corresponsing input (assumed to be in `inputs/` directory, named in the format `dayN`, where N is the day number).

Example runs:

```
echo "Time:        58     81     96     76
Distance:   434   1041   2219   1218" | ./run.sh 6
```

```
./solve.sh 7 --release
```

The inputs are obviously not included in the repository.

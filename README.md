# aoc2025
Advent of code 2025 solutions

## Build

Run `cargo build --profile=release` in the repository's root, then run `cp target/release/aoc-2025 .` to copy the resulting binary to the root.

## Running

General instructions:
```shell
$ ./aoc-2025 --help
Usage: aoc-2025 [COMMAND]

Commands:
  run   Adds favorite command
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

For running added solutions, use the `run` command:

```shell
$ ./aoc-2025 run --help
Adds favorite command

Usage: aoc-2025 run --challenge-id <CHALLENGE_ID>

Options:
  -c, --challenge-id <CHALLENGE_ID>
  -h, --help                         Print help
```

For example, to run the solution of the second challenge of Day 2 (challenge ID `2b`), run the following command:

```shell
$ ./aoc-2025 run -c 2b
343

```

Solutions are expected to print the solution answer at the end of its execution.

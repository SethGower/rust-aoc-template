# rust-aoc-template
Template repository for [AoC](https://adventofcode.com/) challenges done in
Rust. This is based on a template created by [Scoder12](https://github.com/Scoder12)
on [repl.it](https://repl.it/@Scoder12/aoc-rust-template).

## Usage

[aocf](https://github.com/nuxeh/aocf) is required to use run these. This is a
wrapper that accesses the AoC website in order to get the inputs. I found this
project and thought it was cool. The reason aocf is used is to make sure the
most up to date input data is being used. A fall back (instead of `aocf`) is to
specify a filename of the input data. To do this, use the command line options
of either `--file <FILE>` or `-f <FILE>`

In order to add your puzzle solutions, add a file in `src/` called `day<num>.rs`
(or whatever you want to name it). There need to be two functions (publicly
available) for the two parts of each puzzle. These functions should return
`Option<String>`, where the `String` is the answer to the puzzle. An example
configuration is shown in the example below.

```rust
pub fn part1(input: String) -> Option<String> {
    None
}
pub fn part2(input: String) -> Option<String> {
    None
}
```

In order to submit, you can run the program with the flag `--submit` or `-s`.
This will automatically submit the puzzle output, if `aocf` has been properly
configured. 

The main program takes an argument for the day number. So for example, to submit
the puzzle solution for Day 5, you could run. 

```
cargo run -- 5 --submit
```

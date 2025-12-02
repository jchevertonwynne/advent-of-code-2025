# Advent of Code 2025

My repository with a solution boilerplate generator built in

I may or may not write fast solutions instead of nice solutions

## Running days

- `$ cargo run --bin day01` to use real input
- `$ TEST=1 cargo run --bin day01` OR
- `$ cargo run --bin day01 -- -t` OR
- `$ cargo run --bin day01 -- --test` to use test input

## `aoc` solution stub generator installation

`$ cargo install --path . --bin aoc`

## `aoc` usage

`$ aoc 2` or `$ cargo run --bin aoc 2` to get input for a particular day 
`$ aoc` or `$ cargo run --bin aoc` to get input for the current day of the month 

For day 2 this will create the following files:

- binary in `src/bin`
- soution in `src/days`
- added to `src/days/mod.rs`
- input files for real & test inputs (real in `AOC_CACHE` and an empty test input file in `test_input`)

If an input is already present it will not reattempt to download it

Generated days are not automatically added to benchmarks

### `aoc` env vars

- `AOC_SESSION` - Your session cookie - required to use `aoc`. You can find this on the network tab in your browser when you press f12.
- `AOC_CACHE` - The location for the local input cache - required to use `aoc` and the day executables.

### `aoc` cmd line args

- `-year` `-y` - year, default current year
- `-overwrite` `-o` by default overwrite fails if a solution file is found, this disables that

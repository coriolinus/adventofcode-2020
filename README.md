# Advent of Code 2020

Solutions to the exercises at <https://adventofcode.com/2020/>.

This year I'm playing with minimizing scaffolding by generating each day as a subproject
within a shared workspace.

## Per-day setup

```bash
cargo run -- init
```

This will create a new sub-crate and add it to the workspace, as well as downloading the problem's
input. Inputs are saved to a canonical directory. The sub-crate will be named for the day in question,
so it can then be run like

```bash
cargo run -p day01 --part2
```

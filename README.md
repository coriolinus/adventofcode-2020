# Advent of Code 2020

Solutions to the exercises at <https://adventofcode.com/2020/>.

This year I'm playing with minimizing scaffolding by generating each day as a subproject
within a shared workspace.

## Initial setup

Log in to the AoC site with whatever method you prefer. Then use the browser's dev tools to
inspect the cookies. You want the one called `session`. Configure this tool with it,
so it can download the inputs for you.

```bash
cargo run -- config set --session YOUR_SESSION_KEY
```

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

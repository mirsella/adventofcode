# adventofcode

My Advent of Code solutions in Rust, grouped by year and day.

This repo is both a solution archive and a small workflow for starting new puzzle days quickly.

## Layout

- `2023/`: full Rust workspace with one crate per day.
- `2024/`: Rust workspace with shared dependencies for newer days.
- `template/`: `cargo-generate` template for creating a new day crate.

Most days follow the same pattern: embed the puzzle input with `include_str!`, compute `part1`, compute `part2`, and keep example tests next to the code. Harder days sometimes split logic into extra modules or use heavier crates such as `pathfinding`, `rayon`, `regex`, `indexmap`, or `hashbrown`.

## Run One Day

```bash
cargo run --manifest-path 2024/day1/Cargo.toml
```

## Test One Year

```bash
cargo test --manifest-path 2024/Cargo.toml
```

## Template

The `template/` directory is there to bootstrap new puzzle days instead of copying files by hand.

## Notes

- 2023 is the complete set of 25 days.
- 2024 is an in-progress workspace.
- Some days are intentionally rough or experimental because they were written under Advent of Code time pressure.

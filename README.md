# Advent of Code 2018

This is my collection of apps used to solve the [Advent of Code 2018](https://adventofcode.com/2018) puzzles. They are written in Rust 2018 Edition, which currently requires the nightly compiler.

The root is a Cargo workspace with subfolders containing an application for each day. Each app accepts its input data on stdin. To run a particular app, change to the appropriate `dayN` folder and execute the equivalent of the following for your shell:

```sh
cat input | cargo run --release
```

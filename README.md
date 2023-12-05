# Advent of Code Rust Solutions

This repository contains my Rust solutions to the 2023 Advent of Code challenges. Advent of Code is an annual Christmas-themed programming event with a series of puzzles that are released daily in December.

## Project Structure

The repository is organised with each day's solution as a separate Rust library within the corresponding day's folder. The runner executable is able to run and time each day's solution.

## Usage

To run a specific day's solution, use the following commands:

```bash
# Specify the day you'd like to run and provide an input
cargo run 1 input/day1
```

The `input/day1` is an example path for the input file. Adjust the path accordingly based on your file structure.

## Running Tests

The repository includes unit tests for each day's solution. To run the tests for a single day, simply run:

```bash
# Replace '1' with the day number you want to test
cargo test --package day1
```

## License

This repository is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

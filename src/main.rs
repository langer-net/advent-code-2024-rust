// Internal
use advent_code_2024_rust::day_01;
use argparser::ArgParser;

// Std library
use std::process;

fn main() {
    let mut parser = ArgParser::new("Advent of Code 2024");
    parser.add_arg("--test", "This is a test argument");
    parser.add_arg("--test2", "This is a second test argument");
    parser.print_help();

    if let Err(err) = day_01::part1::solve() {
        eprintln!("Application error: {err}");
        process::exit(1);
    };

    if let Err(err) = day_01::part2::solve() {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}

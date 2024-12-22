// Internal
use advent_code_2024_rust::day_01;
use argparser::ArgParser;

// Std library
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[0]);
    let mut parser = ArgParser::new(env::args(), "Advent of Code 2024");
    parser.add_arg("day", "The advent day", true);
    parser.add_arg("part", "The challenge part for the day", true);
    if let Err(err) = parser.parse_args() {
        println!("{}", err);
        process::exit(1);
    };

    if let Err(err) = day_01::part1::solve() {
        eprintln!("Application error: {err}");
        process::exit(1);
    };

    if let Err(err) = day_01::part2::solve() {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}

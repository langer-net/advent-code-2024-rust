// Internal
use advent_code_2024_rust::day_01;
use argparser::ArgParser;

// Std library
use crate::AppErrors::{NoValidCombinationError, ParseArgsError, SolvingFailedError};
use std::error::Error;
use std::{env, process};

enum AppErrors {
    ParseArgsError = 1,
    NoValidCombinationError = 2,
    SolvingFailedError = 3,
}

fn run_day_part(day: &str, part: &str) -> Result<(), Box<dyn Error>> {
    match (day, part) {
        ("01", "1") => day_01::part1::solve(),
        ("01", "2") => day_01::part2::solve(),
        _ => {
            eprintln!("Application error: No valid day & part combination was given.");
            process::exit(NoValidCombinationError as i32);
        }
    }
}

fn main() {
    let mut parser = ArgParser::new(env::args(), "Advent of Code 2024");
    parser.add_arg("day", "The advent day", true);
    parser.add_arg("part", "The challenge part for the day", true);
    if let Err(err) = parser.parse_args() {
        println!("{}", err);
        process::exit(ParseArgsError as i32);
    };

    let day = parser.get("day").unwrap_or("");
    let part = parser.get("part").unwrap_or("");

    if let Err(err) = run_day_part(day, part) {
        eprintln!("Application error: {err}");
        process::exit(SolvingFailedError as i32);
    };
}

// Std library
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load the data.
    let data_path = Path::new("./data/day_01.txt");
    let content = fs::read_to_string(data_path)?;

    // Read get the data per line.
    let lines = content.lines();
    let number_lines = lines.count();

    // Create vectors to catch the numbers.
    let mut numbers_left: Vec<i32> = Vec::with_capacity(number_lines);
    let mut numbers_right: Vec<i32> = Vec::with_capacity(number_lines);

    // Iterate over all lines and fill the vectors.
    for line in content.lines() {
        let mut numbers = line.split_whitespace();
        if let Some(number) = numbers.next() {
            numbers_left.push(number.parse()?);
        }
        if let Some(number) = numbers.next() {
            numbers_right.push(number.parse()?);
        }
    }

    // Sort the vectors.
    numbers_left.sort();
    numbers_right.sort();

    // Calculate the total distance.
    let total_distance: i32 = (0..number_lines)
        .map(|i| (numbers_left[i] - numbers_right[i]).abs())
        .sum();
    println!("Day 01: Part 1: Total distance: {}", total_distance);

    Ok(())
}

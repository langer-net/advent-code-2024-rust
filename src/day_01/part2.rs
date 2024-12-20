use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load the data.
    let data_path = Path::new("./data/day_01_1.txt");
    let content = fs::read_to_string(data_path)?;

    // Read get the data per line.
    let lines = content.lines();
    let number_lines = lines.count();

    // Create vector and map to catch the numbers.
    let mut numbers_left: Vec<i32> = Vec::with_capacity(number_lines);
    let mut counter: HashMap<i32, i32> = HashMap::with_capacity(number_lines);

    // Iterate over all lines and fill the vectors.
    for line in content.lines() {
        let mut numbers = line.split("   ");
        if let Some(number) = numbers.next() {
            numbers_left.push(number.parse()?);
        }
        if let Some(number) = numbers.next() {
            let number = number.parse()?;
            if let Some(count) = counter.get_mut(&number) {
                *count += 1;
            } else {
                counter.insert(number, 1);
            }
        }
    }

    // Calculate the total distance.
    let total_distance = numbers_left
        .iter()
        .fold(0, |acc, num| acc + (num * counter.get(num).unwrap_or(&0)));

    println!("Day 01: Part 2: Total distance: {}", total_distance);

    Ok(())
}

// Std library
use std::error::Error;
use std::fs;
use std::path::Path;

fn check_part(part: &str) -> Option<i32> {
    let parts: Vec<&str> = part.split(",").collect();
    if parts.len() == 2 {
        let numbers: Result<Vec<i32>, _> = parts.into_iter().map(|num| num.parse()).collect();
        return match numbers {
            Ok(numbers) => Some(numbers[0] * numbers[1]),
            Err(_) => None,
        };
    }
    None
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load the data.
    let data_path = Path::new("./data/day_03.txt");
    let content = fs::read_to_string(data_path)?;
    let mut total_sum = 0;

    // Check each string part.
    let parts = content.split("mul(").flat_map(|part| part.split(")"));
    for part in parts {
        if let Some(mul_result) = check_part(part) {
            total_sum += mul_result;
        };
    }

    println!("Day 03: Part 1: Total sum: {}", total_sum);

    Ok(())
}

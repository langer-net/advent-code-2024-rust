// Std library
use std::error::Error;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;

enum Sorting {
    Ascending,
    Descending,
}

fn validate_sorting(sorting: &mut Option<Sorting>, difference: i32) -> bool {
    match difference {
        1..=i32::MAX => match sorting {
            Some(Sorting::Descending) => return false,
            None => *sorting = Some(Sorting::Ascending),
            _ => {}
        },
        i32::MIN..=-1 => match sorting {
            Some(Sorting::Ascending) => return false,
            None => *sorting = Some(Sorting::Descending),
            _ => {}
        },
        _ => {}
    }
    true
}

fn check_if_safe(numbers: Vec<i32>) -> bool {
    if numbers.is_empty() {
        return false;
    }

    // Get the first number.
    let mut numbers = numbers.into_iter();
    let mut previous_number = numbers.next().unwrap();
    let mut sorting: Option<Sorting> = None;

    // Go through
    for number in numbers {
        let difference = number - previous_number;
        previous_number = number;

        if difference.abs() > 3 || difference == 0 {
            return false;
        }

        if !validate_sorting(&mut sorting, difference) {
            return false;
        }
    }

    true
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load the data.
    let data_path = Path::new("./data/day_02.txt");
    let content = fs::read_to_string(data_path)?;

    // Check for each line if it is safe.
    let mut safe_counter = 0;
    for line in content.lines() {
        // Parse the numbers for the current line into a vector.
        let numbers: Result<Vec<i32>, ParseIntError> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>())
            .collect();
        // If numbers are valid, check if the line is safe.
        match numbers {
            Ok(numbers) => {
                if check_if_safe(numbers) {
                    safe_counter += 1
                }
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    println!("Day 02: Part 1: Number of save reports: {}", safe_counter);

    Ok(())
}

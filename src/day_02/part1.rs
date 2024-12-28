// Std library
use std::error::Error;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;

// --- Day 2: Red-Nosed Reports ---
// Fortunately, the first location The Historians want to search isn't a long walk from the Chief Historian's office.
//
// While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they still talk about the time Rudolph was saved through molecular synthesis from a single electron.
//
// They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.
//
// The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:
//
// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9
// This example data contains six reports each containing five levels.
//
// The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:
//
// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
// In the example above, the reports can be found safe or unsafe by checking those rules:
//
// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
// So, in this example, 2 reports are safe.
//
// Analyze the unusual data from the engineers. How many reports are safe?
//
// Your puzzle answer was 224.

enum Sorting {
    Ascending,
    Descending,
}

fn validate_sorting(sorting: &mut Option<Sorting>, difference: i32) -> bool {
    if difference.abs() > 3 || difference == 0 {
        return false;
    }

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

    // Iterate over the numbers.
    for number in numbers {
        let difference = number - previous_number;
        previous_number = number;

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
                    safe_counter += 1;
                }
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    println!("Day 02: Part 1: Number of save reports: {}", safe_counter);

    Ok(())
}

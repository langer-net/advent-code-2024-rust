// Std library
use std::error::Error;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;

// --- Part Two ---
// The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.
//
// The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!
//
// Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.
//
// More of the above example's reports are now safe:
//
// 7 6 4 2 1: Safe without removing any level.
// 1 2 7 8 9: Unsafe regardless of which level is removed.
// 9 7 6 2 1: Unsafe regardless of which level is removed.
// 1 3 2 4 5: Safe by removing the second level, 3.
// 8 6 4 4 1: Safe by removing the third level, 4.
// 1 3 6 7 9: Safe without removing any level.
// Thanks to the Problem Dampener, 4 reports are actually safe!
//
// Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe?
//
// Your puzzle answer was 293.

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

fn check_with_problem_dampener(numbers: Vec<i32>) -> bool {
    for i in 0..numbers.len() {
        let mut modified_numbers = numbers.clone();
        modified_numbers.remove(i);
        if check_if_sequence_safe(&modified_numbers) {
            return true;
        }
    }

    false
}

fn check_if_sequence_safe(numbers: &Vec<i32>) -> bool {
    // Get the first number.
    let mut numbers_iter = numbers.iter();
    let mut previous_number = numbers_iter.next().unwrap();
    let mut sorting: Option<Sorting> = None;

    // Iterate over the numbers.
    for number in numbers_iter {
        let difference = number - previous_number;
        previous_number = number;

        if !validate_sorting(&mut sorting, difference) {
            return false;
        }
    }
    true
}

fn check_if_safe(numbers: Vec<i32>) -> bool {
    if numbers.is_empty() {
        return false;
    }

    // First analysis of the sequence.
    if !check_if_sequence_safe(&numbers) {
        // Recheck the sequence by ignoring one error.
        return check_with_problem_dampener(numbers);
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

    println!("Day 02: Part 2: Number of save reports: {}", safe_counter);

    Ok(())
}

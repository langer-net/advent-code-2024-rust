// Std library
use std::error::Error;
use std::fs;
use std::path::Path;

// --- Day 3: Mull It Over ---
// "Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.
//
// The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"
//
// The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!
//
// It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.
//
// However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.
//
// For example, consider the following section of corrupted memory:
//
// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
// Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).
//
// Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications?
//
// Your puzzle answer was 156388521.

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

    let mul_start_indices: Vec<_> = content
        .match_indices("mul(")
        .map(|(index, _)| index)
        .collect();
    let mul_end_indices: Vec<_> = content.match_indices(")").map(|(index, _)| index).collect();
    let matches = mul_start_indices.iter().map(|&i_start| {
        let i_end = mul_end_indices.iter().find(|&&i| i >= i_start).map(|&i| i);
        // + 4 to remove the "mul(" offset.
        (i_start + 4, i_end.unwrap())
    });

    for i_match in matches {
        if let Some(mul_result) = check_part(&content[i_match.0..i_match.1]) {
            total_sum += mul_result;
        }
    }

    println!("Day 03: Part 1: Total sum: {}", total_sum);

    Ok(())
}

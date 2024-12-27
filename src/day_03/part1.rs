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

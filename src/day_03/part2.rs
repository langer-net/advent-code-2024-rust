// Std library
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

// --- Part Two ---
// As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.
//
// There are two new instructions you'll need to handle:
//
// The do() instruction enables future mul instructions.
// The don't() instruction disables future mul instructions.
// Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.
//
// For example:
//
// xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
// This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.
//
// This time, the sum of the results is 48 (2*4 + 8*5).
//
// Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications?
//
// Your puzzle answer was 75920122.

fn filter_matches<'a>(
    matches: impl Iterator<Item = (usize, Option<&'a usize>)>,
) -> Vec<(usize, usize)> {
    let mut filtered_matches: Vec<(usize, usize)> = Vec::new();
    let mut seen_i_donts = HashSet::new();

    for (i_do, i_dont) in matches {
        if let Some(i_dont_val) = i_dont {
            if seen_i_donts.insert(*i_dont_val) {
                filtered_matches.push((i_do, *i_dont_val));
            }
        }
    }

    filtered_matches
}

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
fn process_match(match_content: &str) -> i32 {
    let mut total_sum = 0;

    let mul_start_indices: Vec<_> = match_content
        .match_indices("mul(")
        .map(|(index, _)| index)
        .collect();
    let mul_end_indices: Vec<_> = match_content
        .match_indices(")")
        .map(|(index, _)| index)
        .collect();
    let matches = mul_start_indices.iter().map(|&i_start| {
        let i_end = mul_end_indices.iter().find(|&&i| i >= i_start).map(|&i| i);
        // + 4 to remove the "mul(" offset.
        (i_start + 4, i_end.unwrap())
    });

    for i_match in matches {
        if let Some(mul_result) = check_part(&match_content[i_match.0..i_match.1]) {
            total_sum += mul_result;
        }
    }

    total_sum
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load the data.
    let data_path = Path::new("./data/day_03.txt");
    let content = fs::read_to_string(data_path)?;
    let mut total_sum = 0;

    // Collect the matching dos and don'ts indices.
    let mut do_indices: Vec<_> = content
        .match_indices("do()")
        .map(|(index, _)| index)
        .collect();
    do_indices.insert(0, 0);
    let dont_indices: Vec<_> = content
        .match_indices("don't()")
        .map(|(index, _)| index)
        .collect();
    let matches = do_indices.iter().map(|&i_do| {
        let i_dont = dont_indices.iter().find(|&&x| x >= i_do);
        (i_do, i_dont)
    });
    let filtered_matches = filter_matches(matches);

    // Iterate over all matches and process the given sequence.
    for i_match in filtered_matches {
        println!(
            "Idx: {:?} | {}",
            i_match,
            &content[i_match.0..i_match.1 + 7]
        );
        let sum = process_match(&content[i_match.0..i_match.1]);
        total_sum += sum;
    }

    println!("Day 03: Part 1: Total sum: {}", total_sum);

    Ok(())
}

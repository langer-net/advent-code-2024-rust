// Std library
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

// --- Part Two ---
// Your analysis only confirmed what everyone feared: the two lists of location IDs are indeed very different.
//
// Or are they?
//
// The Historians can't agree on which group made the mistakes or how to read most of the Chief's handwriting, but in the commotion you notice an interesting detail: a lot of location IDs appear in both lists! Maybe the other numbers aren't location IDs at all but rather misinterpreted handwriting.
//
// This time, you'll need to figure out exactly how often each number from the left list appears in the right list. Calculate a total similarity score by adding up each number in the left list after multiplying it by the number of times that number appears in the right list.
//
// Here are the same example lists again:
//
// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3
// For these example lists, here is the process of finding the similarity score:
//
// The first number in the left list is 3. It appears in the right list three times, so the similarity score increases by 3 * 3 = 9.
// The second number in the left list is 4. It appears in the right list once, so the similarity score increases by 4 * 1 = 4.
// The third number in the left list is 2. It does not appear in the right list, so the similarity score does not increase (2 * 0 = 0).
// The fourth number, 1, also does not appear in the right list.
// The fifth number, 3, appears in the right list three times; the similarity score increases by 9.
// The last number, 3, appears in the right list three times; the similarity score again increases by 9.
// So, for these example lists, the similarity score at the end of this process is 31 (9 + 4 + 0 + 0 + 9 + 9).
//
// Once again consider your left and right lists. What is their similarity score?
//
// Your puzzle answer was 27267728.

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load the data.
    let data_path = Path::new("./data/day_01.txt");
    let content = fs::read_to_string(data_path)?;

    // Read get the data per line.
    let lines = content.lines();
    let number_lines = lines.count();

    // Create vector and map to catch the numbers.
    let mut numbers_left: Vec<i32> = Vec::with_capacity(number_lines);
    let mut counter: HashMap<i32, i32> = HashMap::with_capacity(number_lines);

    // Iterate over all lines and fill the vectors.
    for line in content.lines() {
        let mut numbers = line.split_whitespace();
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

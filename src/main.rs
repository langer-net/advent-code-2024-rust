pub mod day_01 {
    pub mod part1;
    pub mod part2;
}

use std::process;

fn main() {
    if let Err(err) = day_01::part1::solve() {
        eprintln!("Application error: {err}");
        process::exit(1);
    };

    if let Err(err) = day_01::part2::solve() {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}

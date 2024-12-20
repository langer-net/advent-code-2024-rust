pub mod day_01 {
    pub mod part1;
}

use std::process;

fn main() {
    if let Err(err) = day_01::part1::solve() {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}

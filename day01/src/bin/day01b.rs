use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let total_weight: u32 = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.parse::<u32>())
        .filter_map(Result::ok)
        .map(day01::module_total_fuel_req)
        .sum();
    println!("Total weight: {}", total_weight);
}

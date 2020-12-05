use std::io;
use std::io::prelude::*;
use utils::maths;

fn main() {
    let stdin = io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();

    let numbers: Result<Vec<_>, _> = line1.split(',').map(|x| x.parse::<u32>()).collect();
    println!("Hello day03a");
    println!("{}", day03::day2_func());
    println!("{}", maths::add(1, 4));
}

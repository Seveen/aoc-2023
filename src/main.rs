use std::fs;

mod day_1;
mod day_2;

fn main() {
    let Ok(input) = fs::read_to_string("input/day-1.txt") else { return; };
    println!("=== Day 1 ====");
    println!("* Part 1");
    day_1::part_1(input.clone());
    println!("* Part 2");
    day_1::part_2(input);
    
    let Ok(input) = fs::read_to_string("input/day-2.txt") else { return; };
    println!("=== Day 2 ====");
    println!("* Part 1");
    day_2::part_1(input.clone());
    println!("* Part 2");
    day_2::part_2(input);
}

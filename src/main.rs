#![allow(dead_code)]
use std::fs;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let Ok(input) = fs::read_to_string("input/day-4.txt") else { return; };
    day_4::part_2(input);
}

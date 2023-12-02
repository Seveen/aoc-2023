use std::fs;

use day_2::{part_1, part_2};

mod day_1;
mod day_2;

fn main() {
    let Ok(input) = fs::read_to_string("input/day-2.txt") else { return; };
    part_2(input);
}

use adventofcode2023::day15;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day15::puzzle1(&input);
}

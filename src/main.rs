use adventofcode2023::day10;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day10::puzzle1(&input);
}

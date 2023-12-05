use adventofcode2023::day5;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day5::puzzle1(&input);
}

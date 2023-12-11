use adventofcode2023::day11;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day11::puzzle1(&input);
}

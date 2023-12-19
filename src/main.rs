use adventofcode2023::day19;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day19::puzzle1(&input);
}

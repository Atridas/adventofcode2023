use adventofcode2023::day16;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day16::puzzle1(&input);
}

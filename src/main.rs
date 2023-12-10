use adventofcode2023::day7;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day7::puzzle1(&input);
}

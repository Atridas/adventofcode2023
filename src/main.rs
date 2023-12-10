use adventofcode2023::day9;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day9::puzzle1(&input);
}

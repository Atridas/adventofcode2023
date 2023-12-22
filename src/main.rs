use adventofcode2023::day22;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day22::puzzle1(&input);
}

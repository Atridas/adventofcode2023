use adventofcode2023::day20;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day20::puzzle1(&input);
}

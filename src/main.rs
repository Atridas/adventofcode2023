use adventofcode2023::day12;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day12::puzzle2(&input);
}

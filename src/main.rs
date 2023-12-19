use adventofcode2023::day18;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day18::puzzle2(&input);
}

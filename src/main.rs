use adventofcode2023::day13;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day13::puzzle2(&input);
}

use adventofcode2023::day1;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day1::puzzle2(&input);
}

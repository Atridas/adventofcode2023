use adventofcode2023::day3;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day3::puzzle2(&input);
}

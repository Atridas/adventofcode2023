use adventofcode2023::day4;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day4::puzzle2(&input);
}

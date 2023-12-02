use adventofcode2023::day2;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day2::puzzle2(&input);
}

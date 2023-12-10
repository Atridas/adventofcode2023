use adventofcode2023::day8;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day8::puzzle2(&input);
}

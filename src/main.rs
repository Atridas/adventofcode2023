use adventofcode2023::day14;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day14::puzzle2(&input);
}

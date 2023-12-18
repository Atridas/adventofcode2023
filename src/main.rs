use adventofcode2023::day17;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day17::puzzle2(&input);
}

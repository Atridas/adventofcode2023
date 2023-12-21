use adventofcode2023::day21;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day21::puzzle2(&input);
}

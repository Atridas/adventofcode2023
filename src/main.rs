use adventofcode2023::day23;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    day23::puzzle2(&input);
}

use std::collections::HashSet;

pub fn puzzle1(input: &str) {
    let dig_plan = parse(input);

    let mut current = (0, 0);
    let mut digged = HashSet::new();
    digged.insert(current);
    for step in &dig_plan {
        for _ in 0..step.len {
            match step.dir {
                Direction::Down => current.1 += 1,
                Direction::Up => current.1 -= 1,
                Direction::Right => current.0 += 1,
                Direction::Left => current.0 -= 1,
            }
            digged.insert(current);
        }
    }

    let mut to_fill = Vec::new();
    for step in dig_plan {
        for _ in 0..step.len {
            let next = match step.dir {
                Direction::Down => (current.0 - 1, current.1),
                Direction::Up => (current.0 + 1, current.1),
                Direction::Right => (current.0, current.1 + 1),
                Direction::Left => (current.0, current.1 - 1),
            };
            match step.dir {
                Direction::Down => current.1 += 1,
                Direction::Up => current.1 -= 1,
                Direction::Right => current.0 += 1,
                Direction::Left => current.0 -= 1,
            }
            if digged.insert(next) {
                to_fill.push(next);
            }
        }
    }

    while let Some((x, y)) = to_fill.pop() {
        let u = (x, y - 1);
        let d = (x, y + 1);
        let l = (x - 1, y);
        let r = (x + 1, y);

        if digged.insert(u) {
            to_fill.push(u);
        }
        if digged.insert(d) {
            to_fill.push(d);
        }
        if digged.insert(l) {
            to_fill.push(l);
        }
        if digged.insert(r) {
            to_fill.push(r);
        }
    }

    println!("{}", digged.len());
}

pub fn puzzle2(input: &str) {
    let dig_plan = parse(input);

    let mut current = (0, 0);
    let mut area: i64 = 0;
    for i in 0..dig_plan.len() {
        let prev = if i == 0 {
            Direction::Up
        } else {
            dig_plan[i - 1].actual_dir
        };
        let next = if i == dig_plan.len() - 1 {
            Direction::Right
        } else {
            dig_plan[i + 1].actual_dir
        };
        let last_border;
        let next_border;
        match dig_plan[i].actual_dir {
            Direction::Down => {
                last_border = match prev {
                    Direction::Right => (current.0 + 1, current.1),
                    Direction::Left => (current.0 + 1, current.1 + 1),
                    _ => panic!(),
                };
                current.1 += dig_plan[i].actual_len as i64;
                next_border = match next {
                    Direction::Right => (current.0 + 1, current.1),
                    Direction::Left => (current.0 + 1, current.1 + 1),
                    _ => panic!(),
                };
            }
            Direction::Up => {
                last_border = match prev {
                    Direction::Right => (current.0, current.1),
                    Direction::Left => (current.0, current.1 + 1),
                    _ => panic!(),
                };
                current.1 -= dig_plan[i].actual_len as i64;
                next_border = match next {
                    Direction::Right => (current.0, current.1),
                    Direction::Left => (current.0, current.1 + 1),
                    _ => panic!(),
                };
            }
            Direction::Right => {
                last_border = match prev {
                    Direction::Up => (current.0, current.1),
                    Direction::Down => (current.0 + 1, current.1),
                    _ => panic!(),
                };
                current.0 += dig_plan[i].actual_len as i64;
                next_border = match next {
                    Direction::Up => (current.0, current.1),
                    Direction::Down => (current.0 + 1, current.1),
                    _ => panic!(),
                };
            }
            Direction::Left => {
                last_border = match prev {
                    Direction::Up => (current.0, current.1 + 1),
                    Direction::Down => (current.0 + 1, current.1 + 1),
                    _ => panic!(),
                };
                current.0 -= dig_plan[i].actual_len as i64;
                next_border = match next {
                    Direction::Up => (current.0, current.1 + 1),
                    Direction::Down => (current.0 + 1, current.1 + 1),
                    _ => panic!(),
                };
            }
        }

        area += last_border.0 * next_border.1 - last_border.1 * next_border.0;
    }
    area /= 2;
    println!("{area}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct PlanStep {
    dir: Direction,
    len: u32,
    actual_dir: Direction,
    actual_len: u32,
}

fn parse(input: &str) -> Vec<PlanStep> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split_whitespace().collect();
            let dir = match split[0] {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!(""),
            };
            let len: u32 = split[1].parse().unwrap();
            let actual_len = u32::from_str_radix(&split[2][2..7], 16).unwrap();

            let actual_dir = match &split[2][7..8] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!(""),
            };

            PlanStep {
                dir,
                len,
                actual_dir,
                actual_len,
            }
        })
        .collect()
}

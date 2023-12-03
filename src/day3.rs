use std::collections::HashMap;

pub fn puzzle1(input: &str) {
    let mut lines = Vec::new();
    for line in input.lines() {
        lines.push(line.chars().collect::<Vec<_>>());
    }
    let lines = lines;

    let mut acum = 0;

    let mut check_part = |line: usize, first: i32, last: usize, value| {
        let first = if first >= 0 {
            if lines[line][first as usize] != '.' {
                acum += value;
                return;
            }
            first as usize
        } else {
            0
        };
        let last = if last < lines[line].len() {
            if lines[line][last] != '.' {
                acum += value;
                return;
            }
            last
        } else {
            lines[line].len() - 1
        };

        if line > 0 {
            for i in first..=last {
                if lines[line - 1][i] != '.' {
                    acum += value;
                    return;
                }
            }
        }
        if line < lines.len() - 1 {
            for i in first..=last {
                if lines[line + 1][i] != '.' {
                    acum += value;
                    return;
                }
            }
        }
    };

    for i in 0..lines.len() {
        let mut first = None;
        let mut num = 0;
        let line = &lines[i];
        for j in 0..line.len() {
            if line[j].is_numeric() {
                match first {
                    Some(_) => num = num * 10 + line[j] as i32 - '0' as i32,
                    None => {
                        num = line[j] as i32 - '0' as i32;
                        first = Some(j);
                    }
                };
            } else if let Some(f) = first {
                check_part(i, f as i32 - 1, j, num);
                first = None;
                num = 0;
            }
        }
        if let Some(first) = first {
            check_part(i, first as i32 - 1, line.len() + 1, num);
        }
    }

    println!("{acum}");
}

#[derive(PartialEq, Eq, Hash)]
struct Coord(usize, usize);

pub fn puzzle2(input: &str) {
    let mut lines = Vec::new();
    for line in input.lines() {
        lines.push(line.chars().collect::<Vec<_>>());
    }
    let lines = lines;

    let mut gears = HashMap::new();

    let mut add_gear = |coord: Coord, value| {
        match gears.get_mut(&coord) {
            None => {
                gears.insert(coord, vec![value]);
            }
            Some(v) => {
                v.push(value);
            }
        };
    };

    let mut check_part = |line: usize, first: i32, last: usize, value| {
        let first = if first >= 0 {
            if lines[line][first as usize] == '*' {
                add_gear(Coord(line, first as usize), value);
                return;
            }
            first as usize
        } else {
            0
        };
        let last = if last < lines[line].len() {
            if lines[line][last] == '*' {
                add_gear(Coord(line, last), value);
                return;
            }
            last
        } else {
            lines[line].len() - 1
        };

        if line > 0 {
            for i in first..=last {
                if lines[line - 1][i] == '*' {
                    add_gear(Coord(line - 1, i), value);
                    return;
                }
            }
        }
        if line < lines.len() - 1 {
            for i in first..=last {
                if lines[line + 1][i] == '*' {
                    add_gear(Coord(line + 1, i), value);
                    return;
                }
            }
        }
    };

    for i in 0..lines.len() {
        let mut first = None;
        let mut num = 0;
        let line = &lines[i];
        for j in 0..line.len() {
            if line[j].is_numeric() {
                match first {
                    Some(_) => num = num * 10 + line[j] as i32 - '0' as i32,
                    None => {
                        num = line[j] as i32 - '0' as i32;
                        first = Some(j);
                    }
                };
            } else if let Some(f) = first {
                check_part(i, f as i32 - 1, j, num);
                first = None;
                num = 0;
            }
        }
        if let Some(first) = first {
            check_part(i, first as i32 - 1, line.len() + 1, num);
        }
    }

    let mut acum = 0;
    for gear in gears {
        if gear.1.len() == 2 {
            acum += gear.1[0] * gear.1[1];
        }
    }

    println!("{acum}");
}

pub fn puzzle1(input: &str) {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Terrain::Ash,
                    '#' => Terrain::Rocks,
                    _ => panic!("unknown terrain"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut patterns = Vec::new();
    let mut last_pattern = Vec::new();
    for line in input {
        if line.len() > 0 {
            last_pattern.push(line);
        } else {
            patterns.push(last_pattern);
            last_pattern = Vec::new();
        }
    }
    patterns.push(last_pattern);

    let mut vertical_symetries = Vec::new();
    let mut horizontal_symetries = Vec::new();
    // TODO search simetries
    for (i, pattern) in patterns.iter().enumerate() {
        println!("pattern {i}");
        let width = pattern[0].len();
        let height = pattern.len();
        // vertical
        'vert: for i in 1..width {
            for row in 0..height {
                for column in 0..i {
                    let reflect = 2 * i - column - 1;
                    if reflect < width {
                        // println!(
                        //     "{i},{row},{column},{reflect},{:?}{:?}",
                        //     pattern[row][column], pattern[row][reflect]
                        // );
                        if pattern[row][column] != pattern[row][reflect] {
                            continue 'vert;
                        }
                    }
                }
            }
            println!("vertical at {i}");
            vertical_symetries.push(i);
            break;
        }
        // horizontal
        'hori: for i in 1..height {
            for row in 0..i {
                let reflect = 2 * i - row - 1;
                if reflect < height {
                    for column in 0..width {
                        if pattern[row][column] != pattern[reflect][column] {
                            continue 'hori;
                        }
                    }
                }
            }
            println!("horizontal at {i}");
            horizontal_symetries.push(i);
            break;
        }
    }

    let result = horizontal_symetries.iter().fold(0, |a, b| a + b) * 100
        + vertical_symetries.iter().fold(0, |a, b| a + b);

    println!("{:?} {:?}", vertical_symetries, horizontal_symetries);
    println!("{result}");
}

pub fn puzzle2(input: &str) {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Terrain::Ash,
                    '#' => Terrain::Rocks,
                    _ => panic!("unknown terrain"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut patterns = Vec::new();
    let mut last_pattern = Vec::new();
    for line in input {
        if line.len() > 0 {
            last_pattern.push(line);
        } else {
            patterns.push(last_pattern);
            last_pattern = Vec::new();
        }
    }
    patterns.push(last_pattern);

    let mut vertical_symetries = Vec::new();
    let mut horizontal_symetries = Vec::new();
    // TODO search simetries
    for (i, pattern) in patterns.iter().enumerate() {
        println!("pattern {i}");
        let width = pattern[0].len();
        let height = pattern.len();
        // vertical
        'vert: for i in 1..width {
            let mut smudge_found = false;
            for row in 0..height {
                for column in 0..i {
                    let reflect = 2 * i - column - 1;
                    if reflect < width {
                        // println!(
                        //     "{i},{row},{column},{reflect},{:?}{:?}",
                        //     pattern[row][column], pattern[row][reflect]
                        // );
                        if pattern[row][column] != pattern[row][reflect] {
                            if smudge_found {
                                continue 'vert;
                            } else {
                                smudge_found = true;
                            }
                        }
                    }
                }
            }
            if smudge_found {
                println!("vertical at {i}");
                vertical_symetries.push(i);
                break;
            }
        }
        // horizontal
        'hori: for i in 1..height {
            let mut smudge_found = false;
            for row in 0..i {
                let reflect = 2 * i - row - 1;
                if reflect < height {
                    for column in 0..width {
                        if pattern[row][column] != pattern[reflect][column] {
                            if smudge_found {
                                continue 'hori;
                            } else {
                                smudge_found = true;
                            }
                        }
                    }
                }
            }
            if smudge_found {
                println!("horizontal at {i}");
                horizontal_symetries.push(i);
                break;
            }
        }
    }

    let result = horizontal_symetries.iter().fold(0, |a, b| a + b) * 100
        + vertical_symetries.iter().fold(0, |a, b| a + b);

    println!("{result}");
}

#[derive(Debug, PartialEq, Eq)]
enum Terrain {
    Ash,
    Rocks,
}

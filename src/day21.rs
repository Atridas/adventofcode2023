use std::collections::HashSet;

pub fn puzzle1(input: &str) {
    let (map, width, height, starting_position) = parse(input);

    let mut reach = HashSet::new();
    reach.insert(starting_position);
    for _ in 0..64 {
        let mut next_reach = HashSet::new();
        for (x, y) in reach {
            if x > 0 && map[y as usize * width + x as usize - 1] == Tile::Garden {
                next_reach.insert((x - 1, y));
            }
            if (x as usize) < width - 1 && map[y as usize * width + x as usize + 1] == Tile::Garden
            {
                next_reach.insert((x + 1, y));
            }
            if y > 0 && map[y as usize * width - width + x as usize] == Tile::Garden {
                next_reach.insert((x, y - 1));
            }
            if (y as usize) < height - 1
                && map[y as usize * width + width + x as usize] == Tile::Garden
            {
                next_reach.insert((x, y + 1));
            }
        }
        reach = next_reach;
    }

    println!("{}", reach.len());
}

pub fn puzzle2(input: &str) {
    let (map, width, height, starting_position) = parse(input);

    let mut reach = HashSet::new();
    reach.insert(starting_position);
    let mut last_num = 1i64;
    let mut last_dif = vec![0i64, 0i64, 0i64, 0i64, 0i64];
    for i in 1..=500 {
        let mut next_reach = HashSet::new();
        for (x, y) in reach {
            let cx = (x % width as i64 + width as i64) as usize % width;
            let cy = (y % height as i64 + height as i64) as usize % height;
            if map[cy * width + ((cx + width - 1) % width)] == Tile::Garden {
                next_reach.insert((x - 1, y));
            }
            if map[cy * width + ((cx + 1) % width)] == Tile::Garden {
                next_reach.insert((x + 1, y));
            }
            if map[((cy + height - 1) % height) * width + cx] == Tile::Garden {
                next_reach.insert((x, y - 1));
            }
            if map[((cy + 1) % height) * width + cx] == Tile::Garden {
                next_reach.insert((x, y + 1));
            }
        }
        reach = next_reach;

        print!("{i}: {}", reach.len() as i64);
        last_dif[4] = reach.len() as i64 - last_num - last_dif[0] - last_dif[1] - last_dif[3];
        last_dif[3] = reach.len() as i64 - last_num - last_dif[0] - last_dif[1] - last_dif[2];
        last_dif[2] = reach.len() as i64 - last_num - last_dif[0] - last_dif[1];
        last_dif[1] = reach.len() as i64 - last_num - last_dif[0];
        last_dif[0] = reach.len() as i64 - last_num;
        last_num = reach.len() as i64;

        print!(" | {}", last_dif[0]);
        print!(" | {}", last_dif[1]);
        print!(" | {}", last_dif[2]);
        print!(" | {}", last_dif[3]);
        println!(" | {}", last_dif[4]);
    }

    println!("{}", reach.len());
}

fn parse(input: &str) -> (Vec<Tile>, usize, usize, (i64, i64)) {
    let mut starting_position = (0i64, 0i64);
    let mut width = 0;
    let mut height = 0;
    let mut y = 0;

    (
        input
            .lines()
            .map(|line| {
                let mut x = 0;
                let line: Vec<_> = line
                    .chars()
                    .map(|c| {
                        if c == 'S' {
                            starting_position = (x, y);
                            x = x + 1;
                            Tile::Garden
                        } else {
                            x = x + 1;
                            match c {
                                '.' => Tile::Garden,
                                '#' => Tile::Rock,
                                _ => panic!(),
                            }
                        }
                    })
                    .collect();
                y = y + 1;
                assert!(width == 0 || width == line.len());
                width = x as usize;
                height = y as usize;
                line
            })
            .collect::<Vec<_>>()
            .into_iter()
            .fold(Vec::with_capacity(height * width), |mut v, mut row| {
                v.append(&mut row);
                v
            }),
        width,
        height,
        starting_position,
    )
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
}

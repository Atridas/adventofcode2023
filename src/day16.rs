pub fn puzzle1(input: &str) {
    let input = parse(input);

    println!("{}", count_energized(&input, (0, 0, Direction::Right)));
}

pub fn puzzle2(input: &str) {
    let input = parse(input);

    let mut max_energy = 0;
    for x in 0..input.1 {
        let e1 = count_energized(&input, (x, 0, Direction::Down));
        let e2 = count_energized(&input, (x, input.2 - 1, Direction::Up));
        if e1 > max_energy {
            max_energy = e1;
        }
        if e2 > max_energy {
            max_energy = e2;
        }
    }
    for y in 0..input.2 {
        let e1 = count_energized(&input, (0, y, Direction::Right));
        let e2 = count_energized(&input, (input.1 - 1, y, Direction::Left));
        if e1 > max_energy {
            max_energy = e1;
        }
        if e2 > max_energy {
            max_energy = e2;
        }
    }

    println!("{}", max_energy);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mirror {
    Empty,
    ForwardMirror,      // /
    BackwardMirror,     // \
    HorizontalSplitter, // -
    VerticalSplitter,   // |
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}

fn parse(input: &str) -> (Vec<Mirror>, usize, usize) {
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '/' => Mirror::ForwardMirror,
                    '\\' => Mirror::BackwardMirror,
                    '-' => Mirror::HorizontalSplitter,
                    '|' => Mirror::VerticalSplitter,
                    '.' => Mirror::Empty,
                    _ => panic!(""),
                })
                .collect()
        })
        .collect();

    let width = input[0].len();
    let height = input.len();

    let mut field = Vec::with_capacity(width * height);
    for row in input {
        for m in row {
            field.push(m);
        }
    }
    (field, width, height)
}

fn count_energized(
    (input, width, height): &(Vec<Mirror>, usize, usize),
    input_beam: (usize, usize, Direction),
) -> i32 {
    let mut incoming_lightmap = Vec::with_capacity(width * height);
    incoming_lightmap.resize(width * height, Vec::with_capacity(1));
    let mut beams = Vec::with_capacity(10);
    beams.push(input_beam);

    loop {
        match beams.pop() {
            Some((x, y, dir)) => {
                if incoming_lightmap[y * width + x]
                    .iter()
                    .find(|item| *item == &dir)
                    == None
                {
                    incoming_lightmap[y * width + x].push(dir);
                    let nextdir = match input[y * width + x] {
                        Mirror::Empty => (dir, Direction::None),
                        Mirror::ForwardMirror => (
                            match dir {
                                Direction::Down => Direction::Left,
                                Direction::Up => Direction::Right,
                                Direction::Left => Direction::Down,
                                Direction::Right => Direction::Up,
                                _ => panic!(""),
                            },
                            Direction::None,
                        ),
                        Mirror::BackwardMirror => (
                            match dir {
                                Direction::Down => Direction::Right,
                                Direction::Up => Direction::Left,
                                Direction::Left => Direction::Up,
                                Direction::Right => Direction::Down,
                                _ => panic!(""),
                            },
                            Direction::None,
                        ),
                        Mirror::HorizontalSplitter => match dir {
                            Direction::Down => (Direction::Left, Direction::Right),
                            Direction::Up => (Direction::Left, Direction::Right),
                            Direction::Left => (Direction::Left, Direction::None),
                            Direction::Right => (Direction::Right, Direction::None),
                            _ => panic!(""),
                        },
                        Mirror::VerticalSplitter => match dir {
                            Direction::Down => (Direction::Down, Direction::None),
                            Direction::Up => (Direction::Up, Direction::None),
                            Direction::Left => (Direction::Up, Direction::Down),
                            Direction::Right => (Direction::Up, Direction::Down),
                            _ => panic!(""),
                        },
                    };
                    let mut process = |dir| match dir {
                        Direction::Down => {
                            if y < height - 1 {
                                beams.push((x, y + 1, dir));
                            }
                        }
                        Direction::Up => {
                            if y > 0 {
                                beams.push((x, y - 1, dir));
                            }
                        }
                        Direction::Left => {
                            if x > 0 {
                                beams.push((x - 1, y, dir));
                            }
                        }
                        Direction::Right => {
                            if x < width - 1 {
                                beams.push((x + 1, y, dir));
                            }
                        }
                        Direction::None => {}
                    };
                    process(nextdir.0);
                    process(nextdir.1);
                }
            }
            None => break,
        }
    }

    let mut energized = 0;
    for (i, elem) in incoming_lightmap.iter().enumerate() {
        if elem.len() > 0 {
            energized += 1;
            // if elem.len() == 1 {
            //     match elem[0] {
            //         Direction::Down => print!("v"),
            //         Direction::Up => print!("^"),
            //         Direction::Left => print!("<"),
            //         Direction::Right => print!(">"),
            //         _ => panic!(""),
            //     }
            // } else {
            //     print!("{}", elem.len());
            // }
        } else {
            //print!(".");
        }
        if i % width == width - 1 {
            // print!("\n");
        }
    }
    energized
}

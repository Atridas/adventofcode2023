pub fn puzzle1(input: &str) {
    let mut input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Reflector::Empty,
                    'O' => Reflector::Rock,
                    '#' => Reflector::Cube,
                    _ => panic!("unknown terrain"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // TODO transpose?
    let mut acum = 0;
    for j in 0..input.len() {
        for i in 0..input[j].len() {
            if input[j][i] == Reflector::Empty {
                for k in j + 1..input.len() {
                    if input[k][i] == Reflector::Rock {
                        input[j][i] = Reflector::Rock;
                        input[k][i] = Reflector::Empty;
                        acum += input.len() - j;
                        break;
                    } else if input[k][i] == Reflector::Cube {
                        break;
                    }
                }
            } else if input[j][i] == Reflector::Rock {
                acum += input.len() - j;
            }
        }
    }

    println!("{acum}");
}

pub fn puzzle2(input: &str) {
    let size;
    let mut field = Vec::new();
    {
        let input = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Reflector::Empty,
                        'O' => Reflector::Rock,
                        '#' => Reflector::Cube,
                        _ => panic!("unknown terrain"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        size = input.len();
        assert!(size == input[0].len());

        // step 0 is to transpose
        for _ in 0..size {
            field.push(Vec::with_capacity(size));
            field.last_mut().unwrap().resize(size, Reflector::Empty);
        }
        for i in 0..size {
            for j in 0..size {
                field[size - 1 - i][j] = input[j][i];
            }
        }

        for j in 0..field.len() {
            for i in 0..field[j].len() {
                match field[j][i] {
                    Reflector::Empty => print!("."),
                    Reflector::Cube => print!("#"),
                    Reflector::Rock => print!("O"),
                }
            }
            print!("\n");
        }
        print!("\n");
    }

    let mut memory = Vec::new();
    let mut cycle = 0;
    'main_loop: for iteration in 0..1000000000 {
        memory.push(field.clone());
        for _ in 0..4 {
            let mut next = field.clone();
            for j in 0..size {
                'a: for i in 0..size {
                    if field[j][i] == Reflector::Empty {
                        for k in i + 1..size {
                            if field[j][k] == Reflector::Rock {
                                next[i][size - 1 - j] = Reflector::Rock;
                                field[j][k] = Reflector::Empty;
                                continue 'a;
                            } else if field[j][k] == Reflector::Cube {
                                break;
                            }
                        }
                        next[i][size - 1 - j] = Reflector::Empty;
                    } else {
                        next[i][size - 1 - j] = field[j][i];
                    }
                }
            }
            field = next;
        }
        //let mut acum = 0;
        'memories: for m in (0..iteration).rev() {
            for j in 0..size {
                for i in 0..size {
                    if field[j][i] != memory[m][j][i] {
                        continue 'memories;
                    }
                    // match field[size - 1 - i][j] {
                    //     Reflector::Empty => print!("."),
                    //     Reflector::Cube => print!("#"),
                    //     Reflector::Rock => {
                    //         print!("O");
                    //         acum += i + 1
                    //     }
                    // }
                }
                //print!("\n");
            }
            cycle = m;
            break 'main_loop;
        }
        //print!("\n{acum}\n");
    }

    let final_idx = (1000000000 - cycle) % (memory.len() - cycle) + cycle;
    let field = &memory[final_idx];

    let mut acum = 0;
    for j in 0..size {
        for i in 0..size {
            match field[j][i] {
                Reflector::Empty => print!("."),
                Reflector::Cube => print!("#"),
                Reflector::Rock => {
                    print!("O");
                    acum += size - i
                }
            }
            // if field[j][i] == Reflector::Rock {
            //     acum += field.len() - j;
            // }
        }
        print!("\n");
    }

    println!("{acum}");
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Reflector {
    Empty,
    Rock,
    Cube,
}

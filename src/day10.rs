pub fn puzzle1(input: &str) {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::None,
                    '-' => Tile::LR,
                    '|' => Tile::TB,
                    'L' => Tile::TR,
                    'J' => Tile::LT,
                    '7' => Tile::LB,
                    'F' => Tile::RB,
                    'S' => Tile::Starting,
                    _ => panic!("unknown tile"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut initial_coords = (0, 0);
    let mut distances = Vec::new();
    let mut visited = Vec::new();
    for row in 0..input.len() {
        let mut dist = Vec::new();
        let mut vist = Vec::new();
        for column in 0..input[row].len() {
            dist.push(0u64);
            if input[row][column] == Tile::Starting {
                initial_coords = (row, column);
                vist.push(Visit::Pipe);
            } else {
                vist.push(Visit::Unvisited);
            }
        }
        distances.push(dist);
        visited.push(vist);
    }
    let initial_coords = initial_coords;

    let mut path1;
    let mut path2;
    let mut dir1;
    let mut dir2;
    if input[initial_coords.0 - 1][initial_coords.1].connects(Direction::Down) {
        path1 = (initial_coords.0 - 1, initial_coords.1);
        dir1 = Direction::Up;
        if input[initial_coords.0 + 1][initial_coords.1].connects(Direction::Up) {
            path2 = (initial_coords.0 + 1, initial_coords.1);
            dir2 = Direction::Down;
        } else if input[initial_coords.0][initial_coords.1 + 1].connects(Direction::Left) {
            path2 = (initial_coords.0, initial_coords.1 + 1);
            dir2 = Direction::Rigth;
        } else {
            path2 = (initial_coords.0, initial_coords.1 - 1);
            dir2 = Direction::Left;
        }
    } else if input[initial_coords.0 + 1][initial_coords.1].connects(Direction::Up) {
        path1 = (initial_coords.0 + 1, initial_coords.1);
        dir1 = Direction::Down;
        if input[initial_coords.0][initial_coords.1 + 1].connects(Direction::Left) {
            path2 = (initial_coords.0, initial_coords.1 + 1);
            dir2 = Direction::Rigth;
        } else {
            path2 = (initial_coords.0, initial_coords.1 - 1);
            dir2 = Direction::Left;
        }
    } else {
        path1 = (initial_coords.0, initial_coords.1 + 1);
        dir1 = Direction::Rigth;
        path2 = (initial_coords.0, initial_coords.1 - 1);
        dir2 = Direction::Left;
    }

    distances[path1.0][path1.1] = 1;
    distances[path2.0][path2.1] = 1;
    visited[initial_coords.0][initial_coords.1] = Visit::Pipe;
    visited[path1.0][path1.1] = Visit::Pipe;
    visited[path1.0][path1.1] = Visit::Pipe;

    let mut iter = 2;
    loop {
        let sides = input[path1.0][path1.1].sides(dir1);
        for left in sides.0 {
            let coord = left.offset(path1);
            if coord.0 < visited.len()
                && coord.1 < visited[coord.0].len()
                && visited[coord.0][coord.1] == Visit::Unvisited
            {
                visited[coord.0][coord.1] = Visit::Left;
            }
        }
        for right in sides.1 {
            let coord = right.offset(path1);
            if coord.0 < visited.len()
                && coord.1 < visited[coord.0].len()
                && visited[coord.0][coord.1] == Visit::Unvisited
            {
                visited[coord.0][coord.1] = Visit::Right;
            }
        }
        let sides = input[path2.0][path2.1].sides(dir2);
        for left in sides.1 {
            let coord = left.offset(path2);
            if coord.0 < visited.len()
                && coord.1 < visited[coord.0].len()
                && visited[coord.0][coord.1] == Visit::Unvisited
            {
                visited[coord.0][coord.1] = Visit::Left;
            }
        }
        for right in sides.0 {
            let coord = right.offset(path2);
            if coord.0 < visited.len()
                && coord.1 < visited[coord.0].len()
                && visited[coord.0][coord.1] == Visit::Unvisited
            {
                visited[coord.0][coord.1] = Visit::Right;
            }
        }

        dir1 = input[path1.0][path1.1].next(dir1);
        path1 = dir1.offset(path1);
        dir2 = input[path2.0][path2.1].next(dir2);
        path2 = dir2.offset(path2);
        distances[path1.0][path1.1] = iter;
        distances[path2.0][path2.1] = iter;
        visited[path1.0][path1.1] = Visit::Pipe;
        visited[path2.0][path2.1] = Visit::Pipe;
        if path1 == path2 {
            break;
        }
        iter = iter + 1;
    }

    //let distances = distances;
    println!("loop len {iter}");

    let mut to_visit_left = Vec::new();
    let mut to_visit_right = Vec::new();
    for row in 1..visited.len() - 1 {
        for column in 1..visited[row].len() - 1 {
            if visited[row][column] == Visit::Left {
                to_visit_left.push((row, column + 1));
                to_visit_left.push((row, column - 1));
                to_visit_left.push((row + 1, column));
                to_visit_left.push((row - 1, column));
            } else if visited[row][column] == Visit::Right {
                to_visit_right.push((row, column + 1));
                to_visit_right.push((row, column - 1));
                to_visit_right.push((row + 1, column));
                to_visit_right.push((row - 1, column));
            }
        }
    }

    loop {
        match to_visit_left.pop() {
            Some((row, column)) => {
                assert!(visited[row][column] != Visit::Right);
                if visited[row][column] == Visit::Unvisited {
                    visited[row][column] = Visit::Left;
                    to_visit_left.push((row + 1, column));
                    to_visit_left.push((row - 1, column));
                    to_visit_left.push((row, column + 1));
                    to_visit_left.push((row, column - 1));
                }
            }
            None => break,
        };
    }

    loop {
        match to_visit_right.pop() {
            Some((row, column)) => {
                assert!(visited[row][column] != Visit::Left);
                if visited[row][column] == Visit::Unvisited {
                    visited[row][column] = Visit::Right;
                    if row == 0
                        || column == 0
                        || row == visited.len() - 1
                        || column == visited[row].len() - 1
                    {
                    } else {
                        to_visit_right.push((row + 1, column));
                        to_visit_right.push((row - 1, column));
                        to_visit_right.push((row, column + 1));
                        to_visit_right.push((row, column - 1));
                    }
                }
            }
            None => break,
        };
    }

    let enclosed = visited.iter().fold((0, 0), |a, l| {
        l.iter().fold(a, |(l, r), c| match c {
            Visit::Left => (l + 1, r),
            Visit::Right => (l, r + 1),
            _ => (l, r),
        })
    });

    println!("enclosed {}|{}", enclosed.0, enclosed.1);
}

#[derive(PartialEq, Eq)]
enum Tile {
    None,
    LR,
    LT,
    LB,
    TR,
    TB,
    RB,
    Starting,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Rigth,
    None,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Visit {
    Unvisited,
    Pipe,
    Left,
    Right,
}

impl Tile {
    fn connects(&self, dir: Direction) -> bool {
        self.connections().0 == dir || self.connections().1 == dir
    }

    fn connections(&self) -> (Direction, Direction) {
        match self {
            Tile::None => (Direction::None, Direction::None),
            Tile::Starting => (Direction::None, Direction::None),
            Tile::LR => (Direction::Left, Direction::Rigth),
            Tile::LT => (Direction::Left, Direction::Up),
            Tile::LB => (Direction::Left, Direction::Down),
            Tile::TR => (Direction::Up, Direction::Rigth),
            Tile::TB => (Direction::Up, Direction::Down),
            Tile::RB => (Direction::Rigth, Direction::Down),
        }
    }

    fn next(&self, incoming: Direction) -> Direction {
        if self.connections().0 == incoming.opposite() {
            self.connections().1
        } else {
            self.connections().0
        }
    }

    fn sides(&self, incoming: Direction) -> (Vec<Direction>, Vec<Direction>) {
        match self {
            Tile::None => panic!(""),
            Tile::Starting => panic!(""),
            Tile::LR => match incoming {
                Direction::Rigth => (vec![Direction::Up], vec![Direction::Down]),
                Direction::Left => (vec![Direction::Down], vec![Direction::Up]),
                _ => panic!(""),
            },
            Tile::LT => match incoming {
                Direction::Rigth => (vec![], vec![Direction::Rigth, Direction::Down]),
                Direction::Down => (vec![Direction::Rigth, Direction::Down], vec![]),
                _ => panic!(""),
            },
            Tile::LB => match incoming {
                Direction::Rigth => (vec![Direction::Rigth, Direction::Up], vec![]),
                Direction::Up => (vec![], vec![Direction::Rigth, Direction::Up]),
                _ => panic!(""),
            },
            Tile::TR => match incoming {
                Direction::Down => (vec![], vec![Direction::Down, Direction::Left]),
                Direction::Left => (vec![Direction::Down, Direction::Left], vec![]),
                _ => panic!(""),
            },
            Tile::TB => match incoming {
                Direction::Down => (vec![Direction::Rigth], vec![Direction::Left]),
                Direction::Up => (vec![Direction::Left], vec![Direction::Rigth]),
                _ => panic!(""),
            },
            Tile::RB => match incoming {
                Direction::Left => (vec![], vec![Direction::Up, Direction::Left]),
                Direction::Up => (vec![Direction::Up, Direction::Left], vec![]),
                _ => panic!(""),
            },
        }
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::None => Direction::None,
            Direction::Left => Direction::Rigth,
            Direction::Rigth => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    fn offset(&self, (row, column): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::None => (row, column),
            Direction::Left => (row, column.overflowing_sub(1).0),
            Direction::Rigth => (row, column.overflowing_add(1).0),
            Direction::Up => (row.overflowing_sub(1).0, column),
            Direction::Down => (row.overflowing_add(1).0, column),
        }
    }
}

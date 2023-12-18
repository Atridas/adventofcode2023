use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

pub fn puzzle1(input: &str) {
    let (field, heuristics, width, height) = parse(input);

    // for y in 0..height {
    //     for x in 0..width {
    //         print!("{:0>3} ", heuristics[y * width + x]);
    //     }
    //     print!("\n");
    // }

    let mut heap = BinaryHeap::new();
    heap.push(Step::new(
        (1, 0),
        Direction::Right,
        None,
        width,
        &field,
        &heuristics,
    ));
    heap.push(Step::new(
        (0, 1),
        Direction::Down,
        None,
        width,
        &field,
        &heuristics,
    ));

    let mut visited = HashMap::new();

    while let Some(step) = heap.pop() {
        let node = Visited::new(&step);
        if let Some(v) = visited.get(&node) {
            if *v <= step.cost {
                continue;
            }
        }
        visited.insert(node, step.cost);

        // if heap.len() > 10 {
        //     while let Some(step) = heap.pop() {
        //         println!("{:?}", step);
        //     }
        //     return;
        // }
        if step.coord == (width - 1, height - 1) {
            // let mut path = Vec::new();
            // path.push(step.coord);
            // let mut step = &step;
            // while let Some(next) = &step.last {
            //     path.push(next.coord);
            //     step = &next;
            // }

            // for tile in path.iter().rev() {
            //     println!("{}, {}", tile.0, tile.1);
            // }

            println!("{}", step.cost);
            return;
        }
        let can_go_forward = step.path.len() < 3
            || step.path[step.path.len() - 1] != step.path[step.path.len() - 2]
            || step.path[step.path.len() - 1] != step.path[step.path.len() - 3];

        let dir = step.path[step.path.len() - 1];

        if step.coord.0 > 0 && dir != Direction::Right && (can_go_forward || dir != Direction::Left)
        {
            heap.push(Step::new(
                (step.coord.0 - 1, step.coord.1),
                Direction::Left,
                Some(&step),
                width,
                &field,
                &heuristics,
            ));
        }

        if step.coord.0 < width - 1
            && dir != Direction::Left
            && (can_go_forward || dir != Direction::Right)
        {
            heap.push(Step::new(
                (step.coord.0 + 1, step.coord.1),
                Direction::Right,
                Some(&step),
                width,
                &field,
                &heuristics,
            ));
        }

        if step.coord.1 > 0 && dir != Direction::Down && (can_go_forward || dir != Direction::Up) {
            heap.push(Step::new(
                (step.coord.0, step.coord.1 - 1),
                Direction::Up,
                Some(&step),
                width,
                &field,
                &heuristics,
            ));
        }

        if step.coord.1 < height - 1
            && dir != Direction::Up
            && (can_go_forward || dir != Direction::Down)
        {
            heap.push(Step::new(
                (step.coord.0, step.coord.1 + 1),
                Direction::Down,
                Some(&step),
                width,
                &field,
                &heuristics,
            ));
        }
    }
}

pub fn puzzle2(input: &str) {
    let (field, heuristics, width, height) = parse(input);

    let mut heap = BinaryHeap::new();
    heap.push(Step::new(
        (1, 0),
        Direction::Right,
        None,
        width,
        &field,
        &heuristics,
    ));
    heap.push(Step::new(
        (0, 1),
        Direction::Down,
        None,
        width,
        &field,
        &heuristics,
    ));

    let mut visited = HashMap::new();

    while let Some(step) = heap.pop() {
        let node = Visited::new(&step);
        let streak = node.streak;
        let dir = node.direction;
        if let Some(v) = visited.get(&node) {
            if *v <= step.cost {
                continue;
            }
        }
        visited.insert(node, step.cost);

        if step.coord == (width - 1, height - 1) && streak >= 4 {
            let mut acum = 1;
            for i in 1..step.path.len() {
                if step.path[i] == step.path[i - 1] {
                    acum += 1;
                } else {
                    println!("{:?} {acum}", step.path[i - 1]);
                    acum = 1;
                }
            }
            println!("{:?} {acum}", step.path[step.path.len() - 1]);
            println!("{}", step.cost);
            return;
        }

        let mut can_go_right = false;
        let mut can_go_left = false;
        let mut can_go_up = false;
        let mut can_go_down = false;

        if streak >= 4 {
            match dir {
                Direction::Down => {
                    can_go_right = true;
                    can_go_left = true;
                }
                Direction::Up => {
                    can_go_right = true;
                    can_go_left = true;
                }
                Direction::Left => {
                    can_go_up = true;
                    can_go_down = true;
                }
                Direction::Right => {
                    can_go_up = true;
                    can_go_down = true;
                }
            }
        }
        if streak < 10 {
            match dir {
                Direction::Down => {
                    can_go_down = true;
                }
                Direction::Up => {
                    can_go_up = true;
                }
                Direction::Left => {
                    can_go_left = true;
                }
                Direction::Right => {
                    can_go_right = true;
                }
            }
        }

        if step.coord.0 > 0 && can_go_left {
            heap.push(Step::new(
                (step.coord.0 - 1, step.coord.1),
                Direction::Left,
                Some(&step),
                width,
                &field,
                &heuristics,
            ));
        }

        if step.coord.0 < width - 1 && can_go_right {
            heap.push(Step::new(
                (step.coord.0 + 1, step.coord.1),
                Direction::Right,
                Some(&step),
                width,
                &field,
                &heuristics,
            ));
        }

        if step.coord.1 > 0 && can_go_up {
            heap.push(Step::new(
                (step.coord.0, step.coord.1 - 1),
                Direction::Up,
                Some(&step),
                width,
                &field,
                &heuristics,
            ));
        }

        if step.coord.1 < height - 1 && can_go_down {
            heap.push(Step::new(
                (step.coord.0, step.coord.1 + 1),
                Direction::Down,
                Some(&step),
                width,
                &field,
                &heuristics,
            ));
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Step {
    coord: (usize, usize),
    path: Vec<Direction>,
    cost: usize,
    heuristic: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Visited {
    coord: (usize, usize),
    direction: Direction,
    streak: u8,
}

impl Step {
    fn new(
        coord: (usize, usize),
        dir: Direction,
        last: Option<&Step>,
        width: usize,
        field: &Vec<u8>,
        heuristics: &Vec<usize>,
    ) -> Step {
        if let Some(last) = &last {
            let cost = last.cost + field[coord.1 * width + coord.0] as usize;
            let mut path = last.path.clone();
            path.push(dir);
            Self {
                coord,
                cost,
                path,
                heuristic: cost + heuristics[coord.1 * width + coord.0],
            }
        } else {
            let cost = field[coord.1 * width + coord.0] as usize;
            Self {
                coord,
                cost,
                path: vec![dir],
                heuristic: cost + heuristics[coord.1 * width + coord.0],
            }
        }
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> (Vec<u8>, Vec<usize>, usize, usize) {
    let input: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect();

    let width = input[0].len();
    let height = input.len();

    let mut field = Vec::with_capacity(width * height);
    for row in input {
        for m in row {
            field.push(m);
        }
    }

    let mut heuristics = Vec::with_capacity(width * height);
    heuristics.resize(width * height, 0);
    for x in (0..width - 1).rev() {
        heuristics[(height - 1) * width + x] =
            heuristics[(height - 1) * width + x + 1] + field[(height - 1) * width + x + 1] as usize;
    }
    for y in (0..height - 1).rev() {
        heuristics[y * width + width - 1] =
            heuristics[(y + 1) * width + width - 1] + field[(y + 1) * width + width - 1] as usize;
        for x in (0..width - 1).rev() {
            let h1 = heuristics[y * width + x + 1] + field[y * width + x + 1] as usize;
            let h2 = heuristics[(y + 1) * width + x] + field[(y + 1) * width + x] as usize;
            heuristics[y * width + x] = if h1 < h2 { h1 } else { h2 };
        }
    }

    (field, heuristics, width, height)
}

impl Visited {
    fn new(step: &Step) -> Visited {
        let dir = step.path[step.path.len() - 1];

        if step.path.len() == 1 {
            return Self {
                coord: step.coord,
                direction: dir,
                streak: 1,
            };
        }
        for i in 1..step.path.len() - 2 {
            //if step.path.len() - 1 - i < 0 {
            if step.path.len() < 1 + i {
                break;
            }
            if step.path[step.path.len() - 1 - i] != dir {
                return Self {
                    coord: step.coord,
                    direction: dir,
                    streak: i as u8,
                };
            }
        }

        Self {
            coord: step.coord,
            direction: dir,
            streak: step.path.len() as u8,
        }
    }
}

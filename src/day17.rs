use std::{cmp::Ordering, collections::BinaryHeap};

pub fn puzzle1(input: &str) {
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
        Direction::Right,
        None,
        width,
        &field,
        &heuristics,
    ));

    while let Some(step) = heap.pop() {
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
        let mut can_go_forward = true;
        if let Some(prev1) = &step.last {
            if let Some(prev2) = &prev1.last {
                can_go_forward = step.dir != prev1.dir || step.dir != prev2.dir;
            }
        }

        if step.coord.0 > 0
            && step.dir != Direction::Right
            && (can_go_forward || step.dir != Direction::Left)
        {
            heap.push(Step::new(
                (step.coord.0 - 1, step.coord.1),
                Direction::Left,
                Some(Box::new(step.clone())),
                width,
                &field,
                &heuristics,
            ));
        }

        if step.coord.0 < width - 1
            && step.dir != Direction::Left
            && (can_go_forward || step.dir != Direction::Right)
        {
            heap.push(Step::new(
                (step.coord.0 + 1, step.coord.1),
                Direction::Right,
                Some(Box::new(step.clone())),
                width,
                &field,
                &heuristics,
            ));
        }

        if step.coord.1 > 0
            && step.dir != Direction::Down
            && (can_go_forward || step.dir != Direction::Up)
        {
            heap.push(Step::new(
                (step.coord.0, step.coord.1 - 1),
                Direction::Up,
                Some(Box::new(step.clone())),
                width,
                &field,
                &heuristics,
            ));
        }

        if step.coord.1 < height - 1
            && step.dir != Direction::Up
            && (can_go_forward || step.dir != Direction::Down)
        {
            heap.push(Step::new(
                (step.coord.0, step.coord.1 + 1),
                Direction::Down,
                Some(Box::new(step.clone())),
                width,
                &field,
                &heuristics,
            ));
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Step {
    coord: (usize, usize),
    dir: Direction,
    cost: usize,
    heuristic: usize,
    last: Option<Box<Step>>,
}

impl Step {
    fn new(
        coord: (usize, usize),
        dir: Direction,
        last: Option<Box<Step>>,
        width: usize,
        field: &Vec<u8>,
        heuristics: &Vec<usize>,
    ) -> Step {
        let cost = if let Some(last) = &last { last.cost } else { 0 }
            + field[coord.1 * width + coord.0] as usize;
        Self {
            coord,
            dir,
            cost,
            last,
            heuristic: cost + heuristics[coord.1 * width + coord.0],
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
            heuristics[(height - 1) * width + x + 1] + field[(height - 1) * width + x] as usize;
    }
    for y in (0..height - 1).rev() {
        heuristics[y * width + width - 1] =
            heuristics[(y + 1) * width + width - 1] + field[y * width + width - 1] as usize;
        for x in (0..width - 1).rev() {
            let h1 = heuristics[y * width + x + 1];
            let h2 = heuristics[(y + 1) * width + x];
            heuristics[y * width + x] =
                if h1 < h2 { h1 } else { h2 } + field[y * width + x] as usize;
        }
    }

    (field, heuristics, width, height)
}

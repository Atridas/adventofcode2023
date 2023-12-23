use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn puzzle1(input: &str) {
    let (field, width, height) = parse(input);

    let mut paths = BinaryHeap::new();
    paths.push(Path(vec![(1, 0), (1, 1)]));

    let mut finished_paths = Vec::new();

    while let Some(path) = paths.pop() {
        let step = *path.0.last().unwrap();
        if step == (width - 2, height - 1) {
            finished_paths.push(path);
        } else {
            let l = field[step.1 * width + step.0 - 1];
            let r = field[step.1 * width + step.0 + 1];
            let t = field[(step.1 - 1) * width + step.0];
            let b = field[(step.1 + 1) * width + step.0];

            if (l == Tile::Empty || l == Tile::Left) && !path.0.contains(&(step.0 - 1, step.1)) {
                let mut path = path.clone();
                path.0.push((step.0 - 1, step.1));
                paths.push(path);
            }
            if (r == Tile::Empty || r == Tile::Right) && !path.0.contains(&(step.0 + 1, step.1)) {
                let mut path = path.clone();
                path.0.push((step.0 + 1, step.1));
                paths.push(path);
            }
            if (t == Tile::Empty || t == Tile::Up) && !path.0.contains(&(step.0, step.1 - 1)) {
                let mut path = path.clone();
                path.0.push((step.0, step.1 - 1));
                paths.push(path);
            }
            if (b == Tile::Empty || b == Tile::Down) && !path.0.contains(&(step.0, step.1 + 1)) {
                let mut path = path.clone();
                path.0.push((step.0, step.1 + 1));
                paths.push(path);
            }
        }
    }

    let mut max = 0;
    for path in finished_paths {
        max = max.max(path.0.len() - 1);
    }

    println!("{max}");
}

pub fn puzzle2(input: &str) {
    let (field, width, height) = parse(input);

    let corridor = compute_corridor(&field, width, height, Path(vec![(1, 0), (1, 1)])).unwrap();

    let mut nodes = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = Vec::new();
    nodes.insert((1usize, 0usize), vec![(corridor.0, corridor.1)]);
    visited.insert((1usize, 0usize));
    to_visit.push(corridor.0);

    while let Some(node) = to_visit.pop() {
        if node.1 < height - 1 && visited.insert(node) {
            let l = field[node.1 * width + node.0 - 1];
            let r = field[node.1 * width + node.0 + 1];
            let t = field[(node.1 - 1) * width + node.0];
            let b = field[(node.1 + 1) * width + node.0];

            let mut corridors = Vec::new();

            if l != Tile::Wall {
                if let Some(corridor) = compute_corridor(
                    &field,
                    width,
                    height,
                    Path(vec![node, (node.0 - 1, node.1)]),
                ) {
                    corridors.push((corridor.0, corridor.1));
                    to_visit.push(corridor.0);
                }
            }
            if r != Tile::Wall {
                if let Some(corridor) = compute_corridor(
                    &field,
                    width,
                    height,
                    Path(vec![node, (node.0 + 1, node.1)]),
                ) {
                    corridors.push((corridor.0, corridor.1));
                    to_visit.push(corridor.0);
                }
            }
            if t != Tile::Wall {
                if let Some(corridor) = compute_corridor(
                    &field,
                    width,
                    height,
                    Path(vec![node, (node.0, node.1 - 1)]),
                ) {
                    corridors.push((corridor.0, corridor.1));
                    to_visit.push(corridor.0);
                }
            }
            if b != Tile::Wall {
                if let Some(corridor) = compute_corridor(
                    &field,
                    width,
                    height,
                    Path(vec![node, (node.0, node.1 + 1)]),
                ) {
                    corridors.push((corridor.0, corridor.1));
                    to_visit.push(corridor.0);
                }
            }

            nodes.insert(node, corridors);
        }
    }

    let mut paths = Vec::new();
    {
        let mut visited = HashSet::new();
        visited.insert((1usize, 0usize));
        paths.push((visited, (1usize, 0usize), 0usize));
    }

    let mut finished_paths = Vec::new();

    while let Some(path) = paths.pop() {
        if path.1 .1 == width - 1 {
            finished_paths.push(path);
        } else {
            for corridor in nodes.get(&path.1).unwrap() {
                if !path.0.contains(&corridor.0) {
                    let mut path = path.clone();
                    path.0.insert(corridor.0);
                    path.1 = corridor.0;
                    path.2 += corridor.1;
                    paths.push(path);
                }
            }
        }
    }

    let mut max = 0;
    for path in finished_paths {
        max = max.max(path.2);
    }

    println!("{max}");
}

fn compute_corridor(
    field: &Vec<Tile>,
    width: usize,
    height: usize,
    mut path: Path,
) -> Option<((usize, usize), usize)> {
    let step = *path.0.last().unwrap();

    if step.1 == 0 || step.1 == height - 1 {
        return Some((step, path.0.len() - 1));
    }

    let l = field[step.1 * width + step.0 - 1];
    let r = field[step.1 * width + step.0 + 1];
    let t = field[(step.1 - 1) * width + step.0];
    let b = field[(step.1 + 1) * width + step.0];

    let mut next = (0, 0);
    let mut possibilities = 0;

    if (l != Tile::Wall) && !path.0.contains(&(step.0 - 1, step.1)) {
        next = (step.0 - 1, step.1);
        possibilities += 1;
    }
    if (r != Tile::Wall) && !path.0.contains(&(step.0 + 1, step.1)) {
        next = (step.0 + 1, step.1);
        possibilities += 1;
    }
    if (t != Tile::Wall) && !path.0.contains(&(step.0, step.1 - 1)) {
        next = (step.0, step.1 - 1);
        possibilities += 1;
    }
    if (b != Tile::Wall) && !path.0.contains(&(step.0, step.1 + 1)) {
        next = (step.0, step.1 + 1);
        possibilities += 1;
    }

    match possibilities {
        0 => None,
        1 => {
            path.0.push(next);
            compute_corridor(field, width, height, path)
        }
        _ => Some((step, path.0.len() - 1)),
    }
}

fn parse(input: &str) -> (Vec<Tile>, usize, usize) {
    let field: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    '<' => Tile::Left,
                    '>' => Tile::Right,
                    '^' => Tile::Up,
                    'v' => Tile::Down,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let width = field[0].len();
    let height = field.len();
    let mut flat = Vec::with_capacity(width * height);
    for mut row in field {
        flat.append(&mut row);
    }
    assert!(flat.len() == width * height);
    (flat, width, height)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Path(Vec<(usize, usize)>);

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.len().cmp(&self.0.len())
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

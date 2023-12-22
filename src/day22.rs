pub fn puzzle1(input: &str) {
    let (mut bricks, size_x, size_y, size_z) = parse_input(input);

    let mut world = populate_world(&bricks, size_x, size_y, size_z);

    loop {
        if !tick_world(&mut world, &mut bricks, size_x, size_y) {
            break;
        }
    }

    assert!(check_world(&mut world, &mut bricks, size_x, size_y, size_z));

    let mut count = 0;
    'bricks: for (i, brick) in bricks.iter().enumerate() {
        let mut checked = Vec::new();
        for (x, y, z) in &brick.tiles {
            if z + 1 < size_z {
                if let Some(j) = world[((z + 1) * size_y + y) * size_x + x] {
                    if j != i && !checked.contains(&j) {
                        checked.push(j);
                        let mut has_other_support = false;
                        for (x, y, z) in &bricks[j].tiles {
                            if let Some(k) = world[((z - 1) * size_y + y) * size_x + x] {
                                if k != i && k != j {
                                    has_other_support = true;
                                    break;
                                }
                            }
                        }
                        if !has_other_support {
                            continue 'bricks;
                        }
                    }
                }
            }
        }
        count += 1;
    }

    println!("{count}");
}

fn parse_input(input: &str) -> (Vec<Brick>, usize, usize, usize) {
    let bricks: Vec<_> = input
        .lines()
        .map(|line| {
            let mut it = line.split("~");
            let mut begin = it.next().unwrap().split(",");
            let mut end = it.next().unwrap().split(",");

            let x1: usize = begin.next().unwrap().parse().unwrap();
            let y1: usize = begin.next().unwrap().parse().unwrap();
            let z1: usize = begin.next().unwrap().parse().unwrap();
            let x2: usize = end.next().unwrap().parse().unwrap();
            let y2: usize = end.next().unwrap().parse().unwrap();
            let z2: usize = end.next().unwrap().parse().unwrap();

            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);
            let zmin = z1.min(z2);
            let zmax = z1.max(z2);

            let mut tiles = Vec::new();

            for x in xmin..=xmax {
                for y in ymin..=ymax {
                    for z in zmin..=zmax {
                        tiles.push((x, y, z));
                    }
                }
            }

            Brick { tiles }
        })
        .collect();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for brick in &bricks {
        for tile in &brick.tiles {
            if tile.0 > max_x {
                max_x = tile.0;
            }
            if tile.1 > max_y {
                max_y = tile.1;
            }
            if tile.2 > max_z {
                max_z = tile.2;
            }
        }
    }

    (
        bricks,
        (max_x + 1) as usize,
        (max_y + 1) as usize,
        (max_z + 1) as usize,
    )
}

fn populate_world(
    bricks: &Vec<Brick>,
    size_x: usize,
    size_y: usize,
    size_z: usize,
) -> Vec<Option<usize>> {
    let mut world = Vec::new();
    world.resize(size_x * size_y * size_z, None);
    for (i, brick) in bricks.iter().enumerate() {
        for (x, y, z) in &brick.tiles {
            world[(z * size_y + y) * size_x + x] = Some(i);
        }
    }
    world
}

fn check_world(
    world: &mut Vec<Option<usize>>,
    bricks: &mut Vec<Brick>,
    size_x: usize,
    size_y: usize,
    size_z: usize,
) -> bool {
    for (i, brick) in bricks.iter().enumerate() {
        for (x, y, z) in &brick.tiles {
            if world[(z * size_y + y) * size_x + x] != Some(i) {
                return false;
            }
        }
    }

    for z in 0..size_z {
        for y in 0..size_y {
            for x in 0..size_x {
                if let Some(i) = world[(z * size_y + y) * size_x + x] {
                    if !bricks[i].tiles.contains(&(x, y, z)) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn tick_world(
    world: &mut Vec<Option<usize>>,
    bricks: &mut Vec<Brick>,
    size_x: usize,
    size_y: usize,
) -> bool {
    let mut something_fell = false;
    'bricks: for (i, brick) in bricks.iter_mut().enumerate() {
        // check if it can fall
        for (x, y, z) in &brick.tiles {
            if *z == 1
                || (world[((z - 1) * size_y + y) * size_x + x] != None
                    && world[((z - 1) * size_y + y) * size_x + x] != Some(i))
            {
                continue 'bricks;
            }
        }
        // make it fall
        for (x, y, z) in &mut brick.tiles {
            world[(*z * size_y + *y) * size_x + *x] = None;
            *z -= 1;
            world[(*z * size_y + *y) * size_x + *x] = Some(i);
        }
        something_fell = true;
    }

    something_fell
}

struct Brick {
    tiles: Vec<(usize, usize, usize)>,
}

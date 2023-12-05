use std::str::FromStr;

struct Almarac {
    seeds: Vec<u64>,
    maps: Vec<Vec<(u64, u64, u64)>>,
}

impl Almarac {
    fn get_next(&self, map: usize, id: u64) -> u64 {
        for (src_start, dst_start, num) in &self.maps[map] {
            if id >= *src_start && id < src_start + num {
                return dst_start + id - src_start;
            }
        }

        id
    }

    fn get_range(&self, map: usize, ids: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        Vec::new()
    }
}

impl FromStr for Almarac {
    type Err = ();
    fn from_str(input: &str) -> Result<Almarac, ()> {
        let mut lines = input.lines();

        let mut seeds = Vec::new();
        if let Some(seedline) = lines.next() {
            let mut it = seedline.split_whitespace();
            it.next();
            for seed in it {
                seeds.push(seed.parse::<u64>().unwrap());
            }
        }

        //println!("{:?}", seeds);

        lines.next();
        lines.next();

        let mut maps = Vec::new();
        let mut current_map: Vec<(u64, u64, u64)> = Vec::new();

        loop {
            if let Some(line) = lines.next() {
                if line.len() == 0 {
                    current_map.sort_by(|(a, ..), (b, ..)| a.cmp(b));
                    maps.push(current_map);
                    current_map = Vec::new();
                    lines.next();
                } else {
                    let mut params = line.split_whitespace();
                    let dst_start = params.next().unwrap().parse::<u64>().unwrap();
                    let src_start = params.next().unwrap().parse::<u64>().unwrap();
                    let num = params.next().unwrap().parse::<u64>().unwrap();

                    current_map.push((src_start, dst_start, num));
                }
            } else {
                break;
            }
        }

        current_map.sort_by(|(a, ..), (b, ..)| a.cmp(b));
        maps.push(current_map);

        Ok(Almarac { seeds, maps })
    }
}

pub fn puzzle1(input: &str) {
    let almarac = input.parse::<Almarac>().unwrap();

    let mut result = None;
    for seed in &almarac.seeds {
        let mut id = *seed;
        //println!("seed {id}");
        for map in 0..almarac.maps.len() {
            id = almarac.get_next(map, id);
            //println!("to {id}");
        }
        result = Some(match result {
            Some(result) => {
                if result < id {
                    result
                } else {
                    id
                }
            }
            None => id,
        });
    }

    println!("{}", result.unwrap());
}

pub fn puzzle2(input: &str) {
    let almarac = input.parse::<Almarac>().unwrap();

    let mut result = None;
    for i in 0..almarac.seeds.len() / 2 {
        let mut ids = vec![(almarac.seeds[i * 2], almarac.seeds[i * 2])];
        for map in 0..almarac.maps.len() {
            ids = almarac.get_range(map, ids);
        }
        result = Some(match result {
            Some(result) => {
                if result < ids[0].0 {
                    result
                } else {
                    ids[0].0
                }
            }
            None => ids[0].0,
        });
    }

    println!("{}", result.unwrap());
}

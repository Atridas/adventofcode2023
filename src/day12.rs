use std::collections::HashMap;

pub fn puzzle1(input: &str) {
    let records: Vec<_> = input
        .lines()
        .map(|line| {
            let mut wp = line.split_whitespace();
            let springs: Vec<_> = wp
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => panic!("unknown spring condition"),
                })
                .collect();
            let list: Vec<_> = wp
                .next()
                .unwrap()
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            (springs, list)
        })
        .collect();

    let count = records
        .iter()
        .map(|(config, list)| {
            let mut cache = HashMap::new();
            count_combinations(&config, &list, &mut cache)
        })
        .fold(0, |a, b| a + b.unwrap_or(0));

    println!("{count}");
}

pub fn puzzle2(input: &str) {
    let records: Vec<_> = input
        .lines()
        .map(|line| {
            let mut wp = line.split_whitespace();
            let springs: Vec<_> = wp
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => panic!("unknown spring condition"),
                })
                .collect();
            let list: Vec<_> = wp
                .next()
                .unwrap()
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            let mut actual_sprints = Vec::new();
            let mut actual_list = Vec::new();
            actual_sprints.extend(&springs);
            actual_list.extend(&list);
            for _ in 0..4 {
                actual_sprints.push(Spring::Unknown);
                actual_sprints.extend(&springs);
                actual_list.extend(&list);
            }

            (actual_sprints, actual_list)
        })
        .collect();

    let count = records
        .iter()
        .map(|(config, list)| {
            let mut cache = HashMap::new();
            count_combinations(&config, &list, &mut cache)
        })
        .fold(0, |a, b| a + b.unwrap_or(0));

    println!("{count}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Hash, PartialEq, Eq)]
struct DictKey<'a> {
    config: &'a [Spring],
    list: &'a [u32],
}

fn continue_combinations<'a, 'b: 'a>(
    count: u32,
    config: &'b [Spring],
    list: &'b [u32],
    cache: &mut HashMap<DictKey<'a>, Option<u64>>,
) -> Option<u64> {
    if count > 0 {
        if config.len() == 0 || config[0] == Spring::Operational {
            None
        } else {
            continue_combinations(count - 1, &config[1..], &list, cache)
        }
    } else if config.len() == 0 {
        if list.len() == 0 {
            Some(1)
        } else {
            None
        }
    } else if config[0] == Spring::Damaged {
        None
    } else {
        count_combinations(&config[1..], &list, cache)
    }
}

fn count_combinations<'a, 'b: 'a>(
    config: &'b [Spring],
    list: &'b [u32],
    cache: &mut HashMap<DictKey<'a>, Option<u64>>,
) -> Option<u64> {
    if let Some(cached) = cache.get(&DictKey { config, list }) {
        *cached
    } else {
        let value = if list.len() == 0 {
            for spr in config {
                if *spr == Spring::Damaged {
                    return None;
                }
            }
            Some(1)
        } else if config.len() == 0 {
            None
        } else {
            match config[0] {
                Spring::Damaged => {
                    continue_combinations(list[0] - 1, &config[1..], &list[1..], cache)
                }
                Spring::Operational => count_combinations(&config[1..], &list, cache),
                Spring::Unknown => {
                    let damaged = continue_combinations(list[0], &config, &list[1..], cache);
                    let operational = count_combinations(&config[1..], &list, cache);
                    match (damaged, operational) {
                        (Some(a), Some(b)) => Some(a + b),
                        (None, b) => b,
                        (a, None) => a,
                    }
                }
            }
        };
        cache.insert(DictKey { config, list }, value);

        value
    }
}

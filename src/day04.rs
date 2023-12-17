use std::str::FromStr;

struct Scratchcard {
    //id: u32,
    points: u32,
    matches: u32,
    //winning: Vec<u32>,
    //have: Vec<u32>,
}

impl FromStr for Scratchcard {
    type Err = ();
    fn from_str(input: &str) -> Result<Scratchcard, ()> {
        let aux1: Vec<_> = input.split(": ").collect();

        let aux2: Vec<_> = aux1[1].split("|").collect();

        let mut winning = Vec::new();
        //let mut have = Vec::new();
        for winner in aux2[0].split_whitespace() {
            winning.push(winner.parse::<u32>().unwrap());
        }
        let mut points = 0;
        let mut matches = 0;
        for h in aux2[1].split_whitespace() {
            let h: u32 = h.parse().unwrap();
            for winner in &winning {
                if h == *winner {
                    matches += 1;
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                    break;
                }
            }
        }

        Ok(Scratchcard { points, matches })
    }
}

pub fn puzzle1(input: &str) {
    let scratchcards = input
        .lines()
        .map(|line| line.parse::<Scratchcard>().unwrap());
    let mut acum = 0;
    for scratchcard in scratchcards {
        acum += scratchcard.points;
    }
    println!("{acum}");
}

pub fn puzzle2(input: &str) {
    let scratchcards: Vec<_> = input
        .lines()
        .map(|line| line.parse::<Scratchcard>().unwrap())
        .collect();
    let mut acum = 0;
    let mut nums: Vec<u32> = Vec::new();
    for _ in 0..scratchcards.len() {
        nums.push(1);
    }
    for i in 0..scratchcards.len() {
        acum += nums[i];
        for j in 0..scratchcards[i].matches {
            nums[i + j as usize + 1] += nums[i];
        }
    }
    println!("{acum}");
}

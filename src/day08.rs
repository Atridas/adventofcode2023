use gcd::Gcd;
use std::{collections::HashMap, str::FromStr};

pub fn puzzle1(input: &str) {
    let input: Input = input.parse().unwrap();

    let result = input.compute_steps("AAA");
    println!("{result}");
}

pub fn puzzle2(input: &str) {
    let input: Input = input.parse().unwrap();

    let result = input
        .first_nodes
        .iter()
        .map(|initial| input.compute_steps2(initial))
        .fold(1, |a, b| a * b / a.gcd(b))
        * input.instructions as u64;

    //println!("{:?}", results);
    println!("{result}");
}

enum LR {
    Left,
    Right,
}

struct Input {
    first_nodes: Vec<String>,
    steps: HashMap<String, String>,
    instructions: u16,
}

impl Input {
    fn compute_steps(&self, starting: &str) -> usize {
        let mut result = 0;
        let mut current_node = starting;
        loop {
            if current_node == "ZZZ" {
                break;
            }
            current_node = self.steps.get(current_node).unwrap();
            result += self.instructions as usize;
        }
        result
    }
    fn compute_steps2(&self, starting: &str) -> u64 {
        let mut result = 0;
        let mut current_node = starting;
        loop {
            if current_node.ends_with("Z") {
                break;
            }
            current_node = self.steps.get(current_node).unwrap();
            result += 1;
        }
        result
    }
}

impl FromStr for Input {
    type Err = ();
    fn from_str(input: &str) -> Result<Input, ()> {
        let mut lines = input.lines();
        let mut instructions = Vec::new();
        for c in lines.next().unwrap().chars() {
            instructions.push(match c {
                'L' => LR::Left,
                'R' => LR::Right,
                _ => return Err(()),
            });
        }
        let mut nodes = HashMap::new();
        let mut first_nodes = Vec::new();
        lines.next();
        for line in lines {
            let spl: Vec<_> = line.split(" = ").collect();
            let spl2: Vec<_> = spl[1]
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(", ")
                .collect();
            nodes.insert(
                spl[0].to_string(),
                (spl2[0].to_string(), spl2[1].to_string()),
            );
            if spl[0].ends_with("A") {
                first_nodes.push(spl[0].to_string());
            }
        }

        let mut steps = HashMap::new();
        for (node, _) in &nodes {
            let mut dest: &str = &node;
            for instruction in &instructions {
                dest = match *instruction {
                    LR::Left => &nodes.get(dest).unwrap().0,
                    LR::Right => &nodes.get(dest).unwrap().1,
                };
            }
            steps.insert(node.clone(), dest.to_string());
        }

        Ok(Input {
            first_nodes,
            instructions: instructions.len() as u16,
            steps,
        })
    }
}

use std::{collections::HashMap, str::FromStr};

pub fn puzzle1(input: &str) {
    let input: Input = input.parse().unwrap();

    let mut result = 0;
    let mut current_node = &input.first_node;
    loop {
        if current_node == "ZZZ" {
            break;
        }
        for step in &input.instructions {
            current_node = match step {
                LR::Left => &input.nodes.get(current_node).unwrap().0,
                LR::Right => &input.nodes.get(current_node).unwrap().1,
            };
            result += 1;
        }
    }
    println!("{result}");
}

pub fn puzzle2(input: &str) {
    let input: Input = input.parse().unwrap();

    let mut result = 0;
    let mut current_nodes = input.first_nodes;
    'a: loop {
        for step in &input.instructions {
            let mut found = true;

            for current_node in current_nodes.iter_mut() {
                if !current_node.ends_with("Z") {
                    found = false;
                }
                current_node.clone_from(match step {
                    LR::Left => &input.nodes.get(current_node).unwrap().0,
                    LR::Right => &input.nodes.get(current_node).unwrap().1,
                });
            }
            if found {
                break 'a;
            }
            result += 1;
        }
    }
    println!("{result}");
}

enum LR {
    Left,
    Right,
}

struct Input {
    instructions: Vec<LR>,
    first_node: String,
    first_nodes: Vec<String>,
    nodes: HashMap<String, (String, String)>,
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

        Ok(Input {
            first_node: "AAA".to_string(),
            first_nodes,
            instructions,
            nodes,
        })
    }
}

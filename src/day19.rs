use std::{collections::HashMap, str::FromStr};

pub fn puzzle1(input: &str) {
    let (instructions, parts) = parse(input);

    let mut acum = 0;
    for part in parts {
        let mut workflow = "in";
        loop {
            if workflow == "R" {
                break;
            } else if workflow == "A" {
                acum += part.value();
                break;
            } else {
                for rule in instructions.get(workflow).unwrap() {
                    match rule {
                        Rule::GT(cat, q, dst) => {
                            if part.get(*cat) > *q {
                                workflow = dst;
                                break;
                            }
                        }
                        Rule::LT(cat, q, dst) => {
                            if part.get(*cat) < *q {
                                workflow = dst;
                                break;
                            }
                        }
                        Rule::JMP(dst) => {
                            workflow = dst;
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("{acum}");
}

pub fn puzzle2(input: &str) {
    let (instructions, _) = parse(input);

    let mut parts_to_test = Vec::new();
    parts_to_test.push(("in", PartRange::new()));

    //let mut accepted_parts = Vec::new();
    let mut acum = 0;
    while let Some((workflow, mut part_range)) = parts_to_test.pop() {
        if workflow == "R" {
            // ---
        } else if workflow == "A" {
            //accepted_parts.push(part_range);
            acum += part_range.get_combinations();
        } else {
            for rule in instructions.get(workflow).unwrap() {
                match rule {
                    Rule::GT(cat, q, dst) => {
                        if part_range.get(*cat).1 > *q {
                            if part_range.get(*cat).0 < *q {
                                let mut corrected = part_range.clone();
                                corrected.set_min(*cat, *q + 1);
                                parts_to_test.push((*dst, corrected));
                            } else {
                                parts_to_test.push((*dst, part_range));
                                break;
                            }
                            part_range.set_max(*cat, *q);
                        }
                    }
                    Rule::LT(cat, q, dst) => {
                        if part_range.get(*cat).0 < *q {
                            if part_range.get(*cat).1 > *q {
                                let mut corrected = part_range.clone();
                                corrected.set_max(*cat, *q - 1);
                                parts_to_test.push((*dst, corrected));
                            } else {
                                parts_to_test.push((*dst, part_range));
                                break;
                            }
                            part_range.set_min(*cat, *q);
                        }
                    }
                    Rule::JMP(dst) => {
                        parts_to_test.push((*dst, part_range));
                        break;
                    }
                }
            }
        }
    }
    println!("{acum}");
}

fn parse<'a>(input: &'a str) -> (HashMap<&str, Vec<Rule<'a>>>, Vec<Part>) {
    let mut it = input.split("\r\n\r\n");

    let instructions = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let braket = line.find("{").unwrap();
            let name = &line[0..braket];
            let rules: Vec<_> = line[braket + 1..line.len() - 1]
                .split(",")
                .map(|rule| {
                    if let Some(gt) = rule.find(">") {
                        let colon = rule.find(":").unwrap();
                        Rule::GT(
                            rule[0..gt].parse().unwrap(),
                            rule[gt + 1..colon].parse().unwrap(),
                            &rule[colon + 1..],
                        )
                    } else if let Some(lt) = rule.find("<") {
                        let colon = rule.find(":").unwrap();
                        Rule::LT(
                            rule[0..lt].parse().unwrap(),
                            rule[lt + 1..colon].parse().unwrap(),
                            &rule[colon + 1..],
                        )
                    } else {
                        Rule::JMP(rule)
                    }
                })
                .collect();
            (name, rules)
        })
        .collect();
    let parts = it
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (instructions, parts)
}

#[derive(Debug, Clone, Copy)]
enum Category {
    Ex,
    Musical,
    Aerodynamic,
    Shiny,
}

impl FromStr for Category {
    type Err = ();
    fn from_str(input: &str) -> Result<Category, ()> {
        match input {
            "x" => Ok(Category::Ex),
            "m" => Ok(Category::Musical),
            "a" => Ok(Category::Aerodynamic),
            "s" => Ok(Category::Shiny),
            _ => Err(()),
        }
    }
}

struct Part {
    ex: u32,
    musical: u32,
    aerodynamic: u32,
    shiny: u32,
}

impl Part {
    fn value(&self) -> u32 {
        self.ex + self.musical + self.aerodynamic + self.shiny
    }
    fn get(&self, category: Category) -> u32 {
        match category {
            Category::Ex => self.ex,
            Category::Musical => self.musical,
            Category::Aerodynamic => self.aerodynamic,
            Category::Shiny => self.shiny,
        }
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    ex: (u32, u32),
    musical: (u32, u32),
    aerodynamic: (u32, u32),
    shiny: (u32, u32),
}

impl PartRange {
    fn new() -> Self {
        Self {
            ex: (1, 4000),
            musical: (1, 4000),
            aerodynamic: (1, 4000),
            shiny: (1, 4000),
        }
    }
    fn get(&self, category: Category) -> (u32, u32) {
        match category {
            Category::Ex => self.ex,
            Category::Musical => self.musical,
            Category::Aerodynamic => self.aerodynamic,
            Category::Shiny => self.shiny,
        }
    }
    fn set_min(&mut self, category: Category, v: u32) {
        match category {
            Category::Ex => self.ex.0 = v,
            Category::Musical => self.musical.0 = v,
            Category::Aerodynamic => self.aerodynamic.0 = v,
            Category::Shiny => self.shiny.0 = v,
        }
    }
    fn set_max(&mut self, category: Category, v: u32) {
        match category {
            Category::Ex => self.ex.1 = v,
            Category::Musical => self.musical.1 = v,
            Category::Aerodynamic => self.aerodynamic.1 = v,
            Category::Shiny => self.shiny.1 = v,
        }
    }

    fn get_combinations(&self) -> u64 {
        (self.ex.1 - self.ex.0 + 1) as u64
            * (self.musical.1 - self.musical.0 + 1) as u64
            * (self.aerodynamic.1 - self.aerodynamic.0 + 1) as u64
            * (self.shiny.1 - self.shiny.0 + 1) as u64
    }
}

impl FromStr for Part {
    type Err = ();
    fn from_str(input: &str) -> Result<Part, ()> {
        let mut it = input[1..input.len() - 1].split(",");
        let exstr = it.next().unwrap();
        let musicalstr = it.next().unwrap();
        let aerodynamicstr = it.next().unwrap();
        let shinystr = it.next().unwrap();

        let ex = exstr[exstr.find("=").unwrap() + 1..].parse().unwrap();
        let musical = musicalstr[musicalstr.find("=").unwrap() + 1..]
            .parse()
            .unwrap();
        let aerodynamic = aerodynamicstr[aerodynamicstr.find("=").unwrap() + 1..]
            .parse()
            .unwrap();
        let shiny = shinystr[shinystr.find("=").unwrap() + 1..].parse().unwrap();

        Ok(Self {
            ex,
            musical,
            aerodynamic,
            shiny,
        })
    }
}

enum Rule<'a> {
    GT(Category, u32, &'a str),
    LT(Category, u32, &'a str),
    JMP(&'a str),
}

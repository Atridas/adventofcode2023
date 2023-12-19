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

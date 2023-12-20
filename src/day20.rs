use std::collections::HashMap;

pub fn puzzle1(input: &str) {
    let mut modules = parse(input);

    let mut highs = 0;
    let mut lows = 0;
    for _ in 0..1000 {
        // missing 'b' is totally intentional!
        let mut pulses = vec![("roadcaster", "", Pulse::Low)];
        while !pulses.is_empty() {
            let mut next_pulses = Vec::new();
            for (module, origin, pulse) in &pulses {
                match *pulse {
                    Pulse::High => highs += 1,
                    Pulse::Low => lows += 1,
                }
                match modules.get_mut(module) {
                    Some(Module::Broadcast(broadcast)) => {
                        for dst in &broadcast.dsts {
                            next_pulses.push((*dst, *module, *pulse));
                        }
                    }
                    Some(Module::FlipFlop(flipflop)) => {
                        if *pulse == Pulse::Low {
                            let pulse = if flipflop.onoff {
                                flipflop.onoff = false;
                                Pulse::Low
                            } else {
                                flipflop.onoff = true;
                                Pulse::High
                            };
                            for dst in &flipflop.dsts {
                                next_pulses.push((*dst, *module, pulse));
                            }
                        }
                    }
                    Some(Module::Conjunction(conjunction)) => {
                        conjunction.inputs.insert(origin, *pulse);
                        let pulse = if conjunction
                            .inputs
                            .values()
                            .all(|pulse| *pulse == Pulse::High)
                        {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };
                        for dst in &conjunction.dsts {
                            next_pulses.push((*dst, *module, pulse));
                        }
                    }
                    None => {}
                }
            }
            pulses = next_pulses;
        }
    }

    println!("highs:{highs} lows:{lows} mul:{}", highs * lows);
}

fn parse(input: &str) -> HashMap<&str, Module> {
    let mut inputs: HashMap<&str, Vec<&str>> = HashMap::new();

    let mut add_input = |module, dst| {
        if let Some(inputs) = inputs.get_mut(dst) {
            inputs.push(module);
        } else {
            inputs.insert(dst, vec![module]);
        }
    };

    let mut modules: HashMap<&str, Module> = input
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let def = split.next().unwrap();
            let dst = split.next().unwrap();
            for dst in dst.split(", ") {
                add_input(&def[1..], dst);
            }
            if def == "broadcaster" {
                (&def[1..], Module::Broadcast(Broadcast::new(dst)))
            } else if def.starts_with('%') {
                (&def[1..], Module::FlipFlop(FlipFlop::new(dst)))
            } else if def.starts_with('&') {
                (&def[1..], Module::Conjunction(Conjunction::new(dst)))
            } else {
                panic!();
            }
        })
        .collect();

    for (name, module) in &mut modules {
        if let Module::Conjunction(conjunction) = module {
            if let Some(inputs) = inputs.get(*name) {
                for input in inputs {
                    conjunction.inputs.insert(input, Pulse::Low);
                }
            }
        }
    }

    modules
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

struct FlipFlop<'a> {
    onoff: bool,
    dsts: Vec<&'a str>,
}

struct Conjunction<'a> {
    inputs: HashMap<&'a str, Pulse>,
    dsts: Vec<&'a str>,
}

struct Broadcast<'a> {
    dsts: Vec<&'a str>,
}

enum Module<'a> {
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>),
    Broadcast(Broadcast<'a>),
}

impl<'a> FlipFlop<'a> {
    fn new(dsts: &'a str) -> FlipFlop<'a> {
        Self {
            onoff: false,
            dsts: dsts.split(", ").collect(),
        }
    }
}

impl<'a> Conjunction<'a> {
    fn new(dsts: &'a str) -> Conjunction<'a> {
        Self {
            inputs: HashMap::new(),
            dsts: dsts.split(", ").collect(),
        }
    }
}

impl<'a> Broadcast<'a> {
    fn new(dsts: &'a str) -> Broadcast<'a> {
        Self {
            dsts: dsts.split(", ").collect(),
        }
    }
}

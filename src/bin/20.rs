use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster {
        name: String,
        outputs: Vec<String>,
    },
    FlipFlop {
        name: String,
        state: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        name: String,
        inputs: HashMap<String, Pulse>,
        outputs: Vec<String>,
    },
}

impl Module {
    fn outputs(&self) -> &[String] {
        match self {
            Module::Broadcaster { outputs, .. } => outputs,
            Module::FlipFlop { outputs, .. } => outputs,
            Module::Conjunction { outputs, .. } => outputs,
        }
    }

    fn send(&mut self, pulse: Pulse, from: &str, queue: &mut VecDeque<(String, String, Pulse)>) {
        match self {
            Module::Broadcaster { name, outputs } => {
                for output in outputs {
                    queue.push_back((name.to_string(), output.to_string(), pulse));
                }
            }

            Module::FlipFlop {
                name,
                state,
                outputs,
            } => match pulse {
                Pulse::High => {}
                Pulse::Low => {
                    let pulse = match state {
                        false => Pulse::High,
                        true => Pulse::Low,
                    };

                    for output in outputs {
                        queue.push_back((name.to_string(), output.to_string(), pulse));
                    }

                    *state = !*state;
                }
            },

            Module::Conjunction {
                name,
                inputs,
                outputs,
            } => {
                inputs.insert(from.to_string(), pulse);

                let pulse = match inputs.iter().all(|(_, pulse)| pulse == &Pulse::High) {
                    true => Pulse::Low,
                    false => Pulse::High,
                };

                for output in outputs {
                    queue.push_back((name.clone(), output.clone(), pulse));
                }
            }
        }
    }
}

fn parse_module_config(input: &str) -> HashMap<String, Module> {
    let mut module_config = input
        .lines()
        .map(|line| {
            let (label, outputs) = line.trim().split_once(" -> ").unwrap();
            let outputs = outputs.split(", ").map(String::from).collect::<Vec<_>>();

            let (name, module) = if label == "broadcaster" {
                (
                    label.to_string(),
                    Module::Broadcaster {
                        name: label.to_string(),
                        outputs,
                    },
                )
            } else {
                let name = label[1..].to_string();

                let module = match label.chars().next().unwrap() {
                    '%' => Module::FlipFlop {
                        name: name.clone(),
                        state: false,
                        outputs,
                    },

                    '&' => Module::Conjunction {
                        name: name.clone(),
                        inputs: HashMap::default(),
                        outputs,
                    },

                    _ => panic!("unexpected module type"),
                };

                (name, module)
            };

            (name, module)
        })
        .collect::<HashMap<_, _>>();

    for (name, module) in module_config.clone() {
        for output in module.outputs() {
            if let Some(Module::Conjunction { inputs, .. }) = module_config.get_mut(output) {
                inputs.insert(name.clone(), Pulse::Low);
            }
        }
    }
    module_config
}

pub fn solution1(input: &str) -> i64 {
    let mut module_config = parse_module_config(input);
    let (mut h_pulse, mut l_pulse) = (0, 0);

    for _ in 1..=1000 {
        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::default();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((from, target, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::High => h_pulse += 1,
                Pulse::Low => l_pulse += 1,
            }

            if let Some(module) = module_config.get_mut(&target) {
                module.send(pulse, &from, &mut queue);
            }
        }
    }

    h_pulse * l_pulse
}

pub fn solution2(input: &str) -> i64 {
    let mut module_config = parse_module_config(input);
    let mut tracker: HashMap<String, i64> = HashMap::default();

    let mut presses = 1;

    let previous = module_config
        .iter()
        .find(|(_, module)| module.outputs().contains(&String::from("rx")))
        .map(|(module, _)| module.clone())
        .unwrap();

    loop {
        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::default();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((from, target, pulse)) = queue.pop_front() {
            if let Some(module) = module_config.get_mut(&target) {
                module.send(pulse, &from, &mut queue);

                // The mg node is the only one that outputs to rx, and it is a conjunction.
                // We track the iterations where its states alter and apply the Least Common Multiple (LCM) to them.
                if let Module::Conjunction { name, inputs, .. } = &module {
                    if name == &previous {
                        for (input, last_pulse) in inputs {
                            if !tracker.contains_key(input) && last_pulse == &Pulse::High {
                                tracker.insert(input.to_string(), presses);
                            }
                        }

                        if tracker.len() == inputs.len() {
                            return tracker.values().fold(1, |acc, press| lcm(acc, *press));
                        }
                    }
                }
            }
        }

        presses += 1;
    }
}

fn main() {
    let input = std::fs::read_to_string("./data/20.txt").unwrap();
    println!("Part 1: {}", solution1(&input));
    println!("Part 2: {}", solution2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a"#;

    const CASE2: &str = r#"broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output"#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution1(CASE), 32000000);
        assert_eq!(solution1(CASE2), 11687500);
    }

    #[test]
    fn test_solution_2() {
        let input = std::fs::read_to_string("./data/20.txt").unwrap();
        assert_eq!(solution2(&input), 233283622908263);
    }
}

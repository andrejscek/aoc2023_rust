use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Comparison {
    Lesser,
    Greater,
}

#[derive(Debug, Copy, Clone)]
enum Categ {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
enum Rule {
    Comparison {
        field: Categ,
        comparison: Comparison,
        value: u32,
        destination: String,
    },
    Default {
        destination: String,
    },
}

#[derive(Default, Debug, Copy, Clone)]
struct Shape {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Default, Debug, Copy, Clone)]
struct ShapeRange {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

fn calc_size(ranges: &ShapeRange) -> u64 {
    let mut out = 1;
    out *= ranges.x.1 as u64 - ranges.x.0 as u64 + 1;
    out *= ranges.m.1 as u64 - ranges.m.0 as u64 + 1;
    out *= ranges.a.1 as u64 - ranges.a.0 as u64 + 1;
    out *= ranges.s.1 as u64 - ranges.s.0 as u64 + 1;
    out
}

fn parse_input(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Shape>) {
    let (rule, shape) = input.trim().split_once("\n\n").unwrap();

    let rules_out: HashMap<String, Vec<Rule>> = rule
        .lines()
        .map(|rule| {
            let (name, rule) = rule.split_once('{').unwrap();
            let rule = rule.split_once('}').unwrap().0;

            let rules: Vec<Rule> = rule
                .split(',')
                .map(|i| {
                    if !i.contains(':') {
                        return Rule::Default {
                            destination: i.to_string(),
                        };
                    }

                    let (comparison, destination) = i.split_once(':').unwrap();
                    let field = match &comparison[0..1] {
                        "x" => Categ::X,
                        "m" => Categ::M,
                        "a" => Categ::A,
                        "s" => Categ::S,
                        _ => panic!("Invalid field"),
                    };
                    let comp = match &comparison[1..2] {
                        "<" => Comparison::Lesser,
                        ">" => Comparison::Greater,
                        _ => panic!("Invalid comparison"),
                    };
                    let value = comparison[2..].parse().unwrap();

                    Rule::Comparison {
                        field,
                        comparison: comp,
                        value,
                        destination: destination.to_string(),
                    }
                })
                .collect();

            (name.to_string(), rules)
        })
        .collect();

    let shapes: Vec<Shape> = shape
        .lines()
        .map(|l| {
            let mut x = Shape::default();
            l.trim_matches('{')
                .trim_matches('}')
                .split(',')
                .for_each(|part| {
                    let (field, value) = part.split_once('=').unwrap();
                    let value = value.parse().unwrap();
                    match field {
                        "x" => x.x = value,
                        "m" => x.m = value,
                        "a" => x.a = value,
                        "s" => x.s = value,
                        _ => panic!("Invalid field"),
                    }
                });
            x
        })
        .collect();

    (rules_out, shapes)
}

impl Shape {
    fn get(&self, field: &Categ) -> u32 {
        match field {
            Categ::X => self.x,
            Categ::M => self.m,
            Categ::A => self.a,
            Categ::S => self.s,
        }
    }
}

impl ShapeRange {
    fn get_mut(&mut self, field: &Categ) -> &mut (u32, u32) {
        match field {
            Categ::X => &mut self.x,
            Categ::M => &mut self.m,
            Categ::A => &mut self.a,
            Categ::S => &mut self.s,
        }
    }
}

fn solution1(inp: &str) -> u32 {
    let (rules, shapes) = parse_input(inp);
    let mut out = 0;

    for shape in shapes {
        let mut workflow = "in";

        loop {
            let current_workflow = rules.get(workflow).unwrap();
            for rule in current_workflow {
                match rule {
                    Rule::Comparison {
                        field,
                        comparison,
                        value,
                        destination,
                    } => {
                        let val = shape.get(field);
                        if match comparison {
                            Comparison::Lesser => val < *value,
                            Comparison::Greater => val > *value,
                        } {
                            workflow = destination;
                            break;
                        }
                    }
                    Rule::Default { destination } => {
                        workflow = destination;
                        break;
                    }
                }
            }

            if workflow == "A" {
                out += shape.x + shape.m + shape.a + shape.s;
                break;
            } else if workflow == "R" {
                break;
            }
        }
    }

    out
}

fn solve_b(rules: &HashMap<String, Vec<Rule>>, mut range: ShapeRange, map: &str) -> u64 {
    let mut out = 0;

    let mut common = |range: ShapeRange, destination: &str| {
        if destination == "A" {
            out += calc_size(&range);
        } else if destination != "R" {
            out += solve_b(rules, range, destination);
        }
    };

    for rule in rules.get(map).unwrap() {
        match rule {
            Rule::Comparison {
                field,
                comparison,
                value,
                destination,
            } => {
                let mut new_range = range;
                let val = new_range.get_mut(field);
                let rng = range.get_mut(field);

                match comparison {
                    Comparison::Greater if val.1 > *value => {
                        val.0 = val.0.max(*value + 1);
                        rng.1 = rng.1.min(*value);
                    }
                    Comparison::Lesser if val.0 < *value => {
                        val.1 = val.1.min(*value - 1);
                        rng.0 = rng.0.max(*value);
                    }
                    _ => continue,
                }

                common(new_range, destination);
            }
            Rule::Default { destination } => common(range, destination),
        }
    }

    out
}

fn solution2(inp: &str) -> u64 {
    let (rules, _) = parse_input(inp);
    let range = ShapeRange {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };
    solve_b(&rules, range, "in")
}

fn main() {
    let input = std::fs::read_to_string("./data/19.txt").unwrap();
    println!("Part 1: {}", solution1(&input));
    println!("Part 2: {}", solution2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution1(CASE), 19114);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution2(CASE), 167409079868000);
    }
}

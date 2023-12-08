use std::collections::HashMap;

fn parse(s: &str) -> (HashMap<&str, (&str, &str)>, &str) {
    let (instructions, map_str) = s.split_once("\n\n").unwrap();

    let map: HashMap<_, _> = map_str
        .lines()
        .map(|l| {
            let (pos_str, dsts_str) = l.trim().split_once("=").unwrap();
            let pos_str = pos_str.trim();

            let (l, r) = dsts_str
                .trim_matches(|c| "() ".contains(c))
                .split_once(',')
                .unwrap();

            (pos_str, (l.trim(), r.trim()))
        })
        .collect();

    (map, instructions)
}

fn solution1(s: &str) -> u64 {
    let (map, ins) = parse(s);

    let mut cur = "AAA";
    for (steps, c) in ins.chars().cycle().enumerate() {
        let (l, r) = map[cur];
        match c {
            'L' => cur = l,
            'R' => cur = r,
            _ => unreachable!(),
        }
        if cur == "ZZZ" {
            return steps as u64 + 1;
        }
    }
    unreachable!()
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn solution2(s: &str) -> u64 {
    let (map, ins) = parse(s);

    let mut poses: Vec<_> = map
        .keys()
        .filter(|p| p.ends_with('A'))
        .map(|p| *p)
        .collect();
    let mut cycles = vec![];

    for p in &mut poses {
        for (steps, c) in ins.chars().cycle().enumerate() {
            let (l, r) = map[p];
            match c {
                'L' => *p = l,
                'R' => *p = r,
                _ => unreachable!(),
            }

            if p.ends_with('Z') {
                cycles.push(steps as u64 + 1);
                break;
            }
        }
    }

    cycles.iter().fold(1, |acc, c| lcm(*c, acc))
}

fn main() {
    let input = std::fs::read_to_string("./data/8.txt").unwrap();

    let sol1 = solution1(&input);
    println!("Solution 1: {}", sol1);

    let sol2 = solution2(&input);
    println!("Solution 2: {}", sol2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE1: &str = r#"LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)"#;

    const CASE2: &str = r#"LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"#;

    #[test]
    fn test_solution1() {
        let sol = solution1(CASE1);
        assert_eq!(sol, 6);
    }

    #[test]
    fn test_solution2() {
        let sol = solution2(CASE2);
        assert_eq!(sol, 6);
    }
}

use pathfinding::directed::bfs::bfs_reach;
use std::collections::{HashMap, HashSet, VecDeque};

fn solution(input: &str) -> usize {
    let mut graph = input
        .lines()
        .flat_map(|line| {
            let (n, os) = line.trim().split_once(": ").unwrap();
            os.split_whitespace()
                .flat_map(move |o| vec![(n, o), (o, n)])
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert_with(HashSet::new).insert(value);
            acc
        });

    for _ in 0..3 {
        let bridge = find_bridge(&graph);
        graph.get_mut(&bridge.0).unwrap().remove(bridge.1);
        graph.get_mut(&bridge.1).unwrap().remove(bridge.0);
    }
    let gl = bfs_reach(*graph.keys().next().unwrap(), |n| graph[n].iter().copied()).count();

    gl * (graph.len() - gl)
}

fn find_bridge<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>) -> (&'a str, &'a str) {
    let mut paths: HashMap<(&str, &str), usize> = HashMap::new();
    for start in graph.keys().copied() {
        let mut to_see = VecDeque::new();
        to_see.push_back(start);
        let mut seen = HashSet::new();
        seen.insert(start);
        while let Some(node) = to_see.pop_front() {
            for n in graph[&node].iter().copied() {
                if !seen.contains(&n) {
                    to_see.push_back(n);
                    seen.insert(n);
                    let edge = if n < node { (n, node) } else { (node, n) };
                    *paths.entry(edge).or_default() += 1;
                }
            }
        }
    }
    paths.into_iter().max_by_key(|&(_, v)| v).unwrap().0
}
fn main() {
    let input = std::fs::read_to_string("./data/25.txt").unwrap();
    println!("Part 1: {}", solution(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr"#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution(CASE), 54);
    }
}

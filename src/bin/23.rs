use std::collections::HashMap;

fn dfs(
    graph: &HashMap<(usize, usize), Vec<(usize, usize, usize)>>,
    seen: &mut Vec<Vec<bool>>,
    (sr, sc): (usize, usize),
) -> Option<usize> {
    if sr == seen.len() - 1 {
        return Some(0);
    }
    let mut max_dist = None;
    for &(rr, cc, d) in &graph[&(sr, sc)] {
        if !seen[rr][cc] {
            seen[rr][cc] = true;
            if let Some(dist) = dfs(graph, seen, (rr, cc)) {
                max_dist = Some(max_dist.unwrap_or(0).max(d + dist))
            }
            seen[rr][cc] = false;
        }
    }
    max_dist
}

fn solution(input: &str, pt2: bool) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let neighbors: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut graph = HashMap::<_, Vec<_>>::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let neighbors: &[_] = match grid[r][c] {
                '#' => continue,
                _ if pt2 => neighbors,
                '.' => neighbors,
                '^' => &neighbors[0..][..1],
                '>' => &neighbors[1..][..1],
                'v' => &neighbors[2..][..1],
                '<' => &neighbors[3..][..1],
                _ => unreachable!(),
            };
            let node = graph.entry((r, c)).or_default();
            for (dr, dc) in neighbors {
                let rr = (r as i32 + dr) as usize;
                let cc = (c as i32 + dc) as usize;
                let Some(&tile) = grid.get(rr).and_then(|row| row.get(cc)) else {
                    continue;
                };
                if tile != '#' {
                    node.push((rr, cc, 1));
                }
            }
        }
    }
    while let Some((&(r, c), _)) = graph.iter().find(|(_, n)| n.len() == 2) {
        let neighbors = graph.remove(&(r, c)).unwrap();
        let (r1, c1, d1) = neighbors[0];
        let (r2, c2, d2) = neighbors[1];
        let n1 = graph.get_mut(&(r1, c1)).unwrap();
        if let Some(i) = n1.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n1[i] = (r2, c2, d1 + d2);
        }
        let n2 = graph.get_mut(&(r2, c2)).unwrap();
        if let Some(i) = n2.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n2[i] = (r1, c1, d1 + d2);
        }
    }
    dfs(
        &graph,
        &mut vec![vec![false; grid[0].len()]; grid.len()],
        (0, 1),
    )
    .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("./data/23.txt").unwrap();
    println!("Part 1: {}", solution(&input, false));
    println!("Part 2: {}", solution(&input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"#.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#"#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution(CASE, false), 94);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution(CASE, true), 154);
    }
}

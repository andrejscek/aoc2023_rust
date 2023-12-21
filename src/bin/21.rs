use std::collections::{HashMap, VecDeque};

fn parse(input: &str) -> (Vec<Vec<char>>, (i64, i64)) {
    let grid = input
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (sr, sc) = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .position(|c| *c == 'S')
                .map(|c| (r as i64, c as i64))
        })
        .unwrap();
    (grid, (sr, sc))
}

fn find_d(r: usize, c: usize, grid: &Vec<Vec<char>>) -> HashMap<(i64, i64, usize, usize), i64> {
    let r_size = grid[0].len();
    let c_size = grid.len();
    let mut dist_map: HashMap<(i64, i64, usize, usize), i64> = HashMap::new();
    let mut que: VecDeque<(i64, i64, usize, usize, i64)> = VecDeque::new();
    que.push_back((0, 0, r, c, 0));

    while let Some((tr, tc, r, c, d)) = que.pop_front() {
        let mut tr = tr;
        let mut tc = tc;
        let mut r = r as i64;
        let mut c = c as i64;

        if r < 0 {
            tr -= 1;
            r += r_size as i64;
        }
        if r >= r_size as i64 {
            tr += 1;
            r -= r_size as i64;
        }
        if c < 0 {
            tc -= 1;
            c += c_size as i64;
        }
        if c >= c_size as i64 {
            tc += 1;
            c -= c_size as i64;
        }

        if !(0 <= r
            && r < r_size as i64
            && 0 <= c
            && c < c_size as i64
            && grid[r as usize][c as usize] != '#')
        {
            continue;
        }

        if dist_map.contains_key(&(tr, tc, r as usize, c as usize)) {
            continue;
        }

        if tr.abs() > 4 || tc.abs() > 4 {
            continue;
        }

        dist_map.insert((tr, tc, r as usize, c as usize), d);

        for &(dr, dc) in &[(-1, 0), (0, 1), (1, 0), (0, -1)] {
            que.push_back((tr, tc, (r + dr) as usize, (c + dc) as usize, d + 1));
        }
    }

    dist_map
}

fn solve(
    d: i64,
    v: i64,
    steps: i64,
    r_size: i64,
    solve_map: &mut HashMap<(i64, i64, i64), i64>,
) -> i64 {
    let amt = (steps - d) / r_size;
    if solve_map.contains_key(&(d, v, steps)) {
        return *solve_map.get(&(d, v, steps)).unwrap();
    }

    let mut ret = 0;
    for x in 1..=amt {
        if d + r_size * x <= steps && (d + r_size * x) % 2 == (steps % 2) {
            ret += if v == 2 { x + 1 } else { 1 };
        }
    }
    solve_map.insert((d, v, steps), ret);
    ret
}

fn solution(input: &str, steps: i64, pt2: bool) -> i64 {
    let (grid, (sr, sc)) = parse(input);

    let r_size = grid[0].len();
    let c_size = grid.len();
    assert!(r_size == c_size);

    let mut ans = 0;
    let dist_map = find_d(sr as usize, sc as usize, &grid);
    let mut solve_map: HashMap<(i64, i64, i64), i64> = HashMap::new();

    let myv = vec![-3, -2, -1, 0, 1, 2, 3];
    let maxv = *myv.iter().max().unwrap();
    let minv = *myv.iter().min().unwrap();
    for r in 0..r_size {
        for c in 0..c_size {
            if dist_map.get(&(0, 0, r, c)).is_none() {
                continue;
            }

            for tr in myv.iter() {
                for tc in myv.iter() {
                    if !pt2 && (*tr != 0 || *tc != 0) {
                        continue;
                    }

                    let d = dist_map.get(&(*tr, *tc, r, c)).unwrap();
                    if *d % 2 == steps % 2 && *d <= steps {
                        ans += 1;
                    }
                    if (*tr == minv || *tr == maxv) && (*tc == minv || *tc == maxv) {
                        ans += solve(*d, 2, steps, r_size as i64, &mut solve_map) as i64;
                    } else if *tr == minv || *tr == maxv || *tc == minv || *tc == maxv {
                        ans += solve(*d, 1, steps, r_size as i64, &mut solve_map) as i64;
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    let input = std::fs::read_to_string("./data/21.txt").unwrap();
    println!("Part 1: {}", solution(&input, 64, false));
    println!("Part 2: {}", solution(&input, 26501365, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ..........."#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution(CASE, 6, false), 16);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution(CASE, 6, true), 16);
        assert_eq!(solution(CASE, 50, true), 1594);
        assert_eq!(solution(CASE, 5000, true), 16733044);
    }
}

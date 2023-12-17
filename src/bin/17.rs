use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    fn next_move(self, dir_count: usize, part2: bool) -> Vec<Self> {
        let mut ret = Vec::with_capacity(3);
        match self {
            Dir::N => {
                if !part2 {
                    if dir_count < 3 {
                        ret.push(Self::N);
                    }
                    ret.push(Self::E);
                    ret.push(Self::W);
                } else {
                    if dir_count < 10 {
                        ret.push(Self::N)
                    }
                    if dir_count >= 4 {
                        ret.push(Self::E);
                        ret.push(Self::W);
                    }
                }
            }
            Dir::S => {
                if !part2 {
                    if dir_count < 3 {
                        ret.push(Self::S);
                    }
                    ret.push(Self::E);
                    ret.push(Self::W);
                } else {
                    if dir_count < 10 {
                        ret.push(Self::S)
                    }
                    if dir_count >= 4 {
                        ret.push(Self::E);
                        ret.push(Self::W);
                    }
                }
            }
            Dir::W => {
                if !part2 {
                    if dir_count < 3 {
                        ret.push(Self::W);
                    }
                    ret.push(Self::N);
                    ret.push(Self::S);
                } else {
                    if dir_count < 10 {
                        ret.push(Self::W)
                    }
                    if dir_count >= 4 {
                        ret.push(Self::N);
                        ret.push(Self::S);
                    }
                }
            }
            Dir::E => {
                if !part2 {
                    if dir_count < 3 {
                        ret.push(Self::E);
                    }
                    ret.push(Self::N);
                    ret.push(Self::S);
                } else {
                    if dir_count < 10 {
                        ret.push(Self::E)
                    }
                    if dir_count >= 4 {
                        ret.push(Self::N);
                        ret.push(Self::S);
                    }
                }
            }
        }
        ret
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize),
    dir: Dir,
    dir_count: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(
    grid: &[Vec<usize>],
    pos: (usize, usize),
    dir: Dir,
    dir_count: usize,
    part2: bool,
) -> Vec<((usize, usize), Dir)> {
    let ret = Vec::new();
    dir.next_move(dir_count, part2)
        .iter()
        .fold(ret, |mut acc, dir| {
            match dir {
                Dir::N if pos.0 > 0 => acc.push(((pos.0 - 1, pos.1), Dir::N)),
                Dir::S if pos.0 < grid.len() - 1 => acc.push(((pos.0 + 1, pos.1), Dir::S)),
                Dir::W if pos.1 > 0 => acc.push(((pos.0, pos.1 - 1), Dir::W)),
                Dir::E if pos.1 < grid[0].len() - 1 => acc.push(((pos.0, pos.1 + 1), Dir::E)),
                _ => {}
            };
            acc
        })
}

fn custom_dijkstras(grid: &[Vec<usize>], part2: bool) -> usize {
    let start = (0, 0);
    let goal = (grid.len() - 1, grid[0].len() - 1);
    let mut dist: HashMap<((usize, usize), Dir, usize), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        pos: start,
        dir: Dir::E, // dummy
        dir_count: 0,
    });

    while let Some(State {
        cost,
        pos,
        dir,
        dir_count,
    }) = heap.pop()
    {
        if pos == goal {
            if part2 && dir_count < 4 {
                continue;
            }
            return cost;
        }

        if let Some(found) = dist.get(&(pos, dir, dir_count)) {
            if *found < cost {
                continue;
            }
        }

        for (new_pos, new_dir) in neighbors(grid, pos, dir, dir_count, part2) {
            let next = State {
                cost: cost + grid[new_pos.0][new_pos.1],
                pos: new_pos,
                dir: new_dir,
                dir_count: if new_dir != dir { 1 } else { dir_count + 1 },
            };

            if let Some(found) = dist.get(&(new_pos, new_dir, next.dir_count)) {
                if next.cost < *found {
                    heap.push(next);
                    dist.insert((new_pos, new_dir, next.dir_count), next.cost);
                }
            } else {
                heap.push(next);
                dist.insert((new_pos, new_dir, next.dir_count), next.cost);
            }
        }
    }
    panic!("No path found");
}

fn solution(inp: &str, pt2: bool) -> usize {
    let grid: Vec<Vec<usize>> = inp
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    custom_dijkstras(&grid, pt2)
}

fn main() {
    let input = std::fs::read_to_string("./data/17.txt").unwrap();
    println!("Part 1: {}", solution(&input, false));
    println!("Part 2: {}", solution(&input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533"#;

    const CASE2: &str = r#"111111111111
    999999999991
    999999999991
    999999999991
    999999999991"#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution(CASE, false), 102);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution(CASE, true), 94);
        assert_eq!(solution(CASE2, true), 71);
    }
}

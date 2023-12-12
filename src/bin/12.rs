use std::collections::HashMap;

fn line_solve(
    dots: &[char],
    blocks: &[usize],
    map: &mut HashMap<(usize, usize, usize), usize>,
    i: usize,
    bi: usize,
    current: usize,
) -> usize {
    let idx = (i, bi, current);

    if let Some(v) = map.get(&idx) {
        return *v;
    }

    if i == dots.len() {
        if bi == blocks.len() && current == 0 {
            return 1;
        } else if bi == blocks.len() - 1 && current == blocks[bi] {
            return 1;
        } else {
            return 0;
        }
    }

    let mut sum = 0;
    for c in ['.', '#'] {
        if dots[i] == '?' || dots[i] == c {
            if c == '.' && current == 0 {
                sum += line_solve(dots, blocks, map, i + 1, bi, 0);
            } else if c == '.' && current > 0 && bi < blocks.len() && blocks[bi] == current {
                sum += line_solve(dots, blocks, map, i + 1, bi + 1, 0);
            } else if c == '#' {
                sum += line_solve(dots, blocks, map, i + 1, bi, current + 1);
            }
        }
    }
    map.insert(idx, sum);

    sum
}

fn solution(inp: &str, p2: bool) -> usize {
    let mut sum = 0;
    let mut map: HashMap<(usize, usize, usize), usize> = HashMap::new();

    for line in inp.lines() {
        let (dots, blocks) = {
            let (dots, blocks) = line.trim().split_once(" ").unwrap();
            if p2 {
                (
                    format!("{}?{}?{}?{}?{}", dots, dots, dots, dots, dots),
                    format!("{},{},{},{},{}", blocks, blocks, blocks, blocks, blocks),
                )
            } else {
                (dots.to_string(), blocks.to_string())
            }
        };

        let blocks: Vec<usize> = blocks.split(',').map(|x| x.parse().unwrap()).collect();
        let dots: Vec<char> = dots.chars().collect();

        map.clear();
        let score = line_solve(&dots, &blocks, &mut map, 0, 0, 0);
        // println!("{:?}, {:?}, {:?}, {:?}", dots, blocks, score, map.len());

        sum += score;
    }
    sum
}

fn main() {
    let input = std::fs::read_to_string("./data/12.txt").unwrap();

    println!("Part 1: {}", solution(&input, false));
    println!("Part 2: {}", solution(&input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1"#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution(CASE, false), 21);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution(CASE, true), 525152);
    }
}

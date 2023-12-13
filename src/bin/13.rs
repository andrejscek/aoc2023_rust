fn solution(inp: &str, p2: bool) -> usize {
    let mut sum = 0;
    let bad_comp = if p2 { 1 } else { 0 };

    for grid_str in inp.split("\n\n") {
        let grid: Vec<Vec<char>> = grid_str
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();

        let row_size = grid.len();
        let col_size = grid[0].len();

        // vertical
        for c in 0..(col_size - 1) {
            let mut imperfect = 0;
            for dc in 0..col_size {
                let left: i32 = c as i32 - dc as i32;
                let right: i32 = c as i32 + dc as i32 + 1;
                if 0 <= left && left < right && right < col_size as i32 {
                    for r in 0..row_size {
                        if grid[r][left as usize] != grid[r][right as usize] {
                            imperfect += 1;
                        }
                    }
                }
            }
            if imperfect == bad_comp {
                sum += c + 1;
            }
        }

        // horizontal
        for r in 0..(row_size - 1) {
            let mut imperfect = 0;
            for dr in 0..row_size {
                let top: i32 = r as i32 - dr as i32;
                let bottom: i32 = r as i32 + dr as i32 + 1;
                if 0 <= top && top < bottom && bottom < row_size as i32 {
                    for c in 0..col_size {
                        if grid[top as usize][c] != grid[bottom as usize][c] {
                            imperfect += 1;
                        }
                    }
                }
            }
            if imperfect == bad_comp {
                sum += 100 * (r + 1);
            }
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("./data/13.txt").unwrap();

    println!("Part 1: {}", solution(&input, false));
    println!("Part 2: {}", solution(&input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.

    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#"#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution(CASE, false), 405);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution(CASE, true), 400);
    }
}

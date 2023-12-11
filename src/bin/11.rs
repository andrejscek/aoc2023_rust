fn get_max_y(gal_map: &Vec<(i64, i64)>) -> i64 {
    gal_map.iter().map(|(y, _)| *y).max().unwrap()
}

fn get_max_x(gal_map: &Vec<(i64, i64)>) -> i64 {
    gal_map.iter().map(|(_, x)| *x).max().unwrap()
}

fn get_manhattan_distance(gal1: &(i64, i64), gal2: &(i64, i64)) -> i64 {
    return (gal1.0 - gal2.0).abs() + (gal1.1 - gal2.1).abs();
}

fn parse_gal_map(input: &str) -> Vec<(i64, i64)> {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter_map(|(column, char)| {
                    if char == '#' {
                        Some((row as i64, column as i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .collect::<Vec<(i64, i64)>>()
}

fn expand_universe(gal_map: &Vec<(i64, i64)>, expand_by: i64) -> Vec<(i64, i64)> {
    let mut exp_gal_map = gal_map.clone();
    let mut y = 0;
    while y < get_max_y(&exp_gal_map) {
        if exp_gal_map.iter().all(|gal| gal.0 != y) {
            exp_gal_map = exp_gal_map
                .iter()
                .map(|gal| {
                    if gal.0 > y {
                        (gal.0 + expand_by - 1, gal.1)
                    } else {
                        *gal
                    }
                })
                .collect();
            y += expand_by - 1
        }
        y += 1;
    }
    let mut x = 0;
    while x < get_max_x(&exp_gal_map) {
        if exp_gal_map.iter().all(|gal| gal.1 != x) {
            exp_gal_map = exp_gal_map
                .iter()
                .map(|gal| {
                    if gal.1 > x {
                        (gal.0, gal.1 + expand_by - 1)
                    } else {
                        *gal
                    }
                })
                .collect();
            x += expand_by - 1
        }
        x += 1;
    }
    return exp_gal_map;
}

fn solution(inp: &str, exp_fac: i64) -> i64 {
    let gal_map = parse_gal_map(inp);
    let exp_gal_map = expand_universe(&gal_map, exp_fac);
    let mut sum_dist = 0;
    (0..exp_gal_map.len()).for_each(|i| {
        (i + 1..exp_gal_map.len()).for_each(|j| {
            sum_dist += get_manhattan_distance(&exp_gal_map[i], &exp_gal_map[j]);
        });
    });
    return sum_dist;
}

fn main() {
    let input = std::fs::read_to_string("./data/11.txt").unwrap();

    println!("Part 1: {}", solution(&input, 2));
    println!("Part 2: {}", solution(&input, 1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#....."#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution(CASE, 2), 374);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution(CASE, 10), 1030);
        assert_eq!(solution(CASE, 100), 8410);
    }
}

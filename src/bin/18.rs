fn calc_area(points: &[(i64, i64)], b: i64) -> i64 {
    let a = points
        .iter()
        .enumerate()
        .map(|(i, &(x, _))| {
            x * (points[(i + 1) % points.len()].1 - points[(i + points.len() - 1) % points.len()].1)
        })
        .sum::<i64>()
        .abs()
        / 2;
    let i = a - b / 2 + 1;
    i + b
}

fn parse_1(parts: &[&str]) -> (i64, i64, i64) {
    let (d, n) = (
        parts[0].chars().last().unwrap(),
        parts[1].parse::<i64>().unwrap(),
    );
    let (dr, dc) = match d {
        'U' => (-1, 0),
        'D' => (1, 0),
        'L' => (0, -1),
        'R' => (0, 1),
        _ => unreachable!(),
    };

    (dr, dc, n)
}

fn parse_2(parts: &[&str]) -> (i64, i64, i64) {
    let (hex, l) = parts[2][2..parts[2].len() - 1].split_at(5);
    let n = u64::from_str_radix(hex, 16).unwrap() as i64;

    match l {
        "0" => (0, 1, n),
        "1" => (1, 0, n),
        "2" => (0, -1, n),
        "3" => (-1, 0, n),
        _ => panic!("Invalid direction char {}", l),
    }
}

fn solution(inp: &str, pt2: bool) -> i64 {
    // sholace formula + Pick's theorum
    let mut points = vec![(0, 0)];
    let mut b = 0;

    for line in inp.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        let (dr, dc, n) = match pt2 {
            false => parse_1(&parts),
            true => parse_2(&parts),
        };

        b += n;
        let (r, c) = points.last().unwrap();
        points.push((r + dr * n, c + dc * n));
    }

    calc_area(&points, b)
}

fn main() {
    let input = std::fs::read_to_string("./data/18.txt").unwrap();
    println!("Part 1: {}", solution(&input, false));
    println!("Part 2: {}", solution(&input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)"#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution(CASE, false), 62);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution(CASE, true), 952408144115);
    }
}

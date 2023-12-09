use itertools::Itertools;

fn solutions(inp: &str) -> (i64, i64) {
    let nums = inp
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    (
        nums.iter().map(|seq| get_next(&seq)).sum(),
        nums.iter().map(|seq| get_prev(&seq)).sum(),
    )
}

fn next_seq(sequence: &Vec<i64>) -> Vec<i64> {
    sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}

fn get_next(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|n| *n == 0) {
        0
    } else {
        seq.last().unwrap() + get_next(&next_seq(seq))
    }
}

fn get_prev(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|n| *n == 0) {
        0
    } else {
        seq.first().unwrap() - get_prev(&next_seq(seq))
    }
}

fn main() {
    let input = std::fs::read_to_string("./data/9.txt").unwrap();
    let (sol1, sol2) = solutions(&input);
    println!("Solution 1: {}", sol1);
    println!("Solution 2: {}", sol2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"#;

    #[test]
    fn test_solution_1() {
        let (sol1, _) = solutions(CASE);
        assert_eq!(sol1, 114);
    }

    #[test]
    fn test_solution_2() {
        let (_, sol2) = solutions(CASE);
        assert_eq!(sol2, 2);
    }
}

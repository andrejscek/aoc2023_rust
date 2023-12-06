fn count_wins(t: u64, d: u64) -> u64 {
    let mut c: u64 = 0;
    for i in 1..t {
        if (t - i) * i > d {
            c += 1;
        }
    }
    c
}

fn solution1(s: &str) -> u64 {
    let mut lines = s.lines();

    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let dists: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let mut prod: u64 = 1;
    for (t, d) in times.iter().zip(dists.iter()) {
        prod *= count_wins(*t, *d);
    }

    prod
}

fn solution2(s: &str) -> u64 {
    let mut lines = s.lines();

    let t: u64 = lines
        .next()
        .unwrap()
        .replace(" ", "")
        .split_once(":")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();

    let d: u64 = lines
        .next()
        .unwrap()
        .replace(" ", "")
        .split_once(":")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();

    count_wins(t, d)
}

fn main() {
    let input = std::fs::read_to_string("./data/6.txt").unwrap();

    let sol1 = solution1(&input);
    println!("Solution 1: {}", sol1);

    let sol2 = solution2(&input);
    println!("Solution 2: {}", sol2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = "Time:      7  15   30
    Distance:  9  40  200";

    #[test]
    fn test_solution1() {
        let sol = solution1(CASE);
        assert_eq!(sol, 288);
        // assert_eq!(sol, 0);
    }

    #[test]
    fn test_solution2() {
        let sol = solution2(CASE);
        assert_eq!(sol, 71503);
    }
}

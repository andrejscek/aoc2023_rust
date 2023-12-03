use itertools::Itertools;
use std::collections::HashMap;

fn solution2(filename: &str) -> i64 {
    let input = std::fs::read_to_string(filename).unwrap();
    let lines = input.lines().collect_vec();
    let (w, h) = (lines[0].len(), lines.len());

    let mut gear_vals: HashMap<_, Vec<_>> = HashMap::new();
    for (y, l) in lines.iter().enumerate() {
        let mut x: usize = 0;
        while x < w {
            let number_len = l[x..].find(|c: char| !c.is_ascii_digit()).unwrap_or(w - x);
            if number_len > 0 {
                let n = l[x..x + number_len].parse::<i64>().unwrap();
                for ny in y as i64 - 1..=y as i64 + 1 {
                    for nx in x as i64 - 1..=(x + number_len) as i64 {
                        if 0 <= ny && ny < h as i64 && 0 <= nx && nx < w as i64 {
                            let b = lines[ny as usize].as_bytes()[nx as usize];
                            if b == b'*' {
                                gear_vals.entry((nx, ny)).or_default().push(n);
                            }
                        }
                    }
                }
            }
            x += number_len + 1;
        }
    }

    let mut prod = 0;
    for val in gear_vals.values() {
        if let &[a, b] = val.as_slice() {
            prod += a * b;
        }
    }
    prod
}

fn solution1(filename: &str) -> i64 {
    let input = std::fs::read_to_string(filename).unwrap();
    let lines = input.lines().collect_vec();
    let (w, h) = (lines[0].len(), lines.len());

    let mut sum = 0;
    for (y, l) in lines.iter().enumerate() {
        let mut x = 0;
        while x < w {
            let number_len = l[x..].find(|c: char| !c.is_ascii_digit()).unwrap_or(w - x);
            if number_len > 0 {
                let n = l[x..x + number_len].parse::<i64>().unwrap();

                let mut is_valid = false;
                for ny in y as i64 - 1..=y as i64 + 1 {
                    for nx in x as i64 - 1..=(x + number_len) as i64 {
                        if 0 <= ny && ny < h as i64 && 0 <= nx && nx < w as i64 {
                            let b = lines[ny as usize].as_bytes()[nx as usize];
                            is_valid |= b != b'.' && !b.is_ascii_digit();
                        }
                    }
                }

                sum += if is_valid { n } else { 0 };
            }
            x += number_len + 1;
        }
    }
    sum
}

fn main() {
    let sol1 = solution1("./data/3.txt");
    println!("Solution 1: {}", sol1);

    let sol2 = solution2("./data/3.txt");
    println!("Solution 2: {}", sol2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let sol = solution1("./data/3t.txt");
        assert_eq!(sol, 4361);
    }

    #[test]
    fn test_solution2() {
        let sol = solution2("./data/3t.txt");
        assert_eq!(sol, 467835);
    }
}

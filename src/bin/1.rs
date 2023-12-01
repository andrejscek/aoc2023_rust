use aoc2023::utils::read_lines;
use std::collections::BTreeMap;

fn solution1(vec: &Vec<String>) -> u32 {
    let mut count: u32 = 0;
    for s in vec {
        let first = s.chars().find(|c| c.is_numeric()).unwrap();
        let last = s.chars().rev().find(|c| c.is_numeric()).unwrap();

        let result = u32::from_str_radix(&format!("{first}{last}"), 10).unwrap_or(0);
        count += result;
    }
    count
}

fn solution2(vec: &Vec<String>) -> u32 {
    let mut res: u32 = 0;
    let valid_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in vec {
        let mut digit_positions = BTreeMap::new();
        for (i, &digit) in valid_digits.iter().enumerate() {
            let digit_as_number = (i + 1) as u8;
            for (i, _) in line.match_indices(digit) {
                digit_positions.insert(i, digit_as_number);
            }
        }
        line.chars().enumerate().for_each(|(i, ch)| {
            if ch.is_numeric() {
                digit_positions.insert(i, ch as u8 - '0' as u8);
            }
        });
        // println!("digit_positions: {:?}", digit_positions);
        let digits = digit_positions.values().cloned().collect::<Vec<u8>>();
        let res_part =
            u32::from_str_radix(&format!("{}{}", digits[0], digits[digits.len() - 1]), 10)
                .unwrap_or(0);
        res += res_part;
    }
    res
}

fn main() {
    let file_path = "./data/1.txt";
    let lines: Vec<String> = read_lines(file_path).expect("wrong file path");

    println!("Solution 1: {}", solution1(&lines));
    println!("Solution 2: {}", solution2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let lines: Vec<String> = read_lines("./data/1_t1.txt").expect("wrong file path");
        assert_eq!(solution1(&lines), 142);
    }

    #[test]
    fn test_solution2() {
        let lines: Vec<String> = read_lines("./data/1_t2.txt").expect("wrong file path");
        assert_eq!(solution2(&lines), 281);
    }
}

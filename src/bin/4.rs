use std::collections::{HashMap, HashSet};

fn solution1(filename: &str) -> u32 {
    let input = std::fs::read_to_string(filename).unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let (win, sel) = line.split_once(":").unwrap().1.split_once("|").unwrap();

        let mut sel_set: HashSet<i32> = HashSet::new();
        for num in sel.trim().split(" ") {
            if let Ok(num) = num.trim().parse::<i32>() {
                sel_set.insert(num);
            }
        }

        let mut score: u32 = 0;
        for num in win.trim().split(" ") {
            if let Ok(num) = num.trim().parse::<i32>() {
                if sel_set.contains(&num) {
                    score += 1;
                }
            }
        }
        if score == 0 {
            continue;
        }
        sum += 2_i64.pow(score - 1) as u32;
    }

    sum
}

fn solution2(filename: &str) -> u32 {
    let input = std::fs::read_to_string(filename).unwrap();

    let mut cards_won: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let (card_no, rest) = line.trim_start_matches("Card ").split_once(":").unwrap();
        let card_no = card_no.trim().parse::<u32>().unwrap();
        let (win, sel) = rest.split_once("|").unwrap();

        let mut sel_set: HashSet<i32> = HashSet::new();
        for num in sel.split_whitespace() {
            if let Ok(num) = num.trim().parse::<i32>() {
                sel_set.insert(num);
            }
        }

        cards_won
            .entry(card_no)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        let mut card: u32 = card_no;
        let card_mult = *cards_won.get(&card_no).unwrap_or(&1);
        for num in win.split_whitespace() {
            if let Ok(num) = num.trim().parse::<i32>() {
                if sel_set.contains(&num) {
                    card += 1;

                    cards_won
                        .entry(card)
                        .and_modify(|e| *e += 1 * card_mult)
                        .or_insert(card_mult);
                }
            }
        }
    }

    cards_won.values().sum::<u32>()
}

fn main() {
    let sol1 = solution1("./data/4.txt");
    println!("Solution 1: {}", sol1);

    let sol2 = solution2("./data/4.txt");
    println!("Solution 2: {}", sol2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let sol = solution1("./data/4t.txt");
        assert_eq!(sol, 13);
    }

    #[test]
    fn test_solution2() {
        let sol = solution2("./data/4t.txt");
        assert_eq!(sol, 30);
    }
}

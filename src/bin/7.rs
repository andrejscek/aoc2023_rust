use std::{cmp::Ordering, fmt::Debug};

const CARDS_P1: &str = "AKQJT98765432";
const CARDS_P2: &str = "AKQT98765432J";

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct Hand {
    cards: Vec<u8>,
    bid: u16,
}

fn parse(input: &str, mappings: &'static str) -> Vec<Hand> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let (cards, bid) = line.trim().split_at(5);

        let cards = cards
            .as_bytes()
            .iter()
            .map(|&c| 13 - mappings.find(c as char).unwrap() as u8)
            .collect();
        let bid = bid.trim().parse().unwrap();

        hands.push(Hand { cards, bid });
    }

    hands
}

fn score_p1(cards: &[u8]) -> HandType {
    let mut counts = [0; 13];
    for &c in cards {
        counts[13 - c as usize] += 1;
    }

    if counts.iter().any(|&c| c == 5) {
        HandType::FiveOfAKind
    } else if counts.iter().any(|&c| c == 4) {
        HandType::FourOfAKind
    } else if counts.iter().any(|&c| c == 3) && counts.iter().any(|&c| c == 2) {
        HandType::FullHouse
    } else if counts.iter().any(|&c| c == 3) {
        HandType::ThreeOfAKind
    } else if counts.iter().filter(|&&c| c == 2).count() == 2 {
        HandType::TwoPair
    } else if counts.iter().any(|&c| c == 2) {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn score_p2(cards: &[u8]) -> HandType {
    let mut counts = [0; 13];
    for &c in cards {
        counts[13 - c as usize] += 1;
    }

    let jacks = counts[12];
    let mut counts = counts[0..12]
        .into_iter()
        .map(|&x| x)
        .filter(|x| *x != 0)
        .collect::<Vec<_>>();
    counts.sort_by_key(|x| *x);
    counts.reverse();

    if counts.len() <= 1 {
        HandType::FiveOfAKind
    } else if counts[0] + jacks == 5 {
        HandType::FiveOfAKind
    } else if counts[0] + jacks == 4 {
        HandType::FourOfAKind
    } else if ((counts[0] + jacks == 3) && (counts[1] == 2))
        || ((counts[0] == 3) && (counts[1] + jacks == 2))
    {
        HandType::FullHouse
    } else if counts[0] + jacks == 3 {
        HandType::ThreeOfAKind
    } else if (counts[0] + jacks == 2 && counts[1] == 2)
        || (counts[0] == 2 && counts[1] + jacks == 2)
    {
        HandType::TwoPair
    } else if counts[0] + jacks == 2 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn score_first(a: &[u8], b: &[u8]) -> Ordering {
    for (&a, &b) in a.iter().zip(b.iter()) {
        if a != b {
            return a.cmp(&b);
        }
    }

    Ordering::Equal
}

fn solution1(s: &str) -> usize {
    let mut hands = parse(s, CARDS_P1);

    hands.sort_by(|a, b| {
        let a_score = score_p1(&a.cards) as u8;
        let b_score = score_p1(&b.cards) as u8;

        if a_score == b_score {
            score_first(&b.cards, &a.cards)
        } else {
            a_score.cmp(&b_score)
        }
    });
    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(x, y)| y.bid as usize * (x + 1))
        .sum::<usize>()
}

fn solution2(s: &str) -> usize {
    let mut hands = parse(s, CARDS_P2);

    hands.sort_by(|a, b| {
        let a_score = score_p2(&a.cards) as u8;
        let b_score = score_p2(&b.cards) as u8;

        if a_score == b_score {
            score_first(&b.cards, &a.cards)
        } else {
            a_score.cmp(&b_score)
        }
    });
    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(x, y)| y.bid as usize * (x + 1))
        .sum::<usize>()
        .into()
}

fn main() {
    let input = std::fs::read_to_string("./data/7.txt").unwrap();

    let sol1 = solution1(&input);
    println!("Solution 1: {}", sol1);

    let sol2 = solution2(&input);
    println!("Solution 2: {}", sol2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    #[test]
    fn test_solution1() {
        let sol = solution1(CASE);
        assert_eq!(sol, 6440);
    }

    #[test]
    fn test_solution2() {
        let sol = solution2(CASE);
        assert_eq!(sol, 5905);
    }
}

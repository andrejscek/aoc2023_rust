use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Round {
    red: usize,
    blue: usize,
    green: usize,
}

impl Round {
    fn new() -> Round {
        Round {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    game_id: usize,
    rounds: Vec<Round>,
}
impl Game {
    fn new() -> Game {
        Game {
            game_id: 0,
            rounds: Vec::new(),
        }
    }
}

fn read_games_from_file(file_path: &str) -> io::Result<Vec<Game>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut games: Vec<Game> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(colon_index) = line.find(':') {
            // Ignore the input before ":"
            let game_rounds_str = &line[colon_index + 1..].trim();
            let game_id: usize = line[..colon_index]
                .trim()
                .trim_start_matches("Game ")
                .parse()
                .unwrap();
            let mut game = Game::new();
            game.game_id = game_id as usize;
            // Split rounds using ";" as a delimiter
            game.rounds = game_rounds_str
                .split(';')
                .map(|round_str| parse_round(round_str))
                .collect();

            // Add rounds to the vector of games
            games.push(game);
        }
    }

    Ok(games)
}

fn parse_round(round_str: &str) -> Round {
    let mut round = Round::new();

    for segment in round_str.split(',') {
        let mut parts = segment.trim().split_whitespace();

        if let Some(count_str) = parts.next() {
            if let Some(color) = parts.next() {
                match color {
                    "red" => round.red += count_str.parse::<usize>().unwrap_or(0),
                    "blue" => round.blue += count_str.parse::<usize>().unwrap_or(0),
                    "green" => round.green += count_str.parse::<usize>().unwrap_or(0),
                    _ => (),
                }
            }
        }
    }

    round
}

fn solution1(games: &Vec<Game>) -> usize {
    let max_vals = Round {
        red: 12,
        blue: 14,
        green: 13,
    };
    let mut valid = 0;

    for game in games.iter() {
        let mut is_valid = true;
        for round in game.rounds.iter() {
            if round.red > max_vals.red
                || round.blue > max_vals.blue
                || round.green > max_vals.green
            {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            valid += game.game_id;
        }
    }
    valid
}

fn solution2(games: &Vec<Game>) -> usize {
    let mut total_power = 0;
    for game in games.iter() {
        let mut max_vals = Round::new();

        for round in game.rounds.iter() {
            if round.red > max_vals.red {
                max_vals.red = round.red;
            }
            if round.blue > max_vals.blue {
                max_vals.blue = round.blue;
            }
            if round.green > max_vals.green {
                max_vals.green = round.green;
            }
        }
        total_power += max_vals.red * max_vals.blue * max_vals.green;
    }
    total_power
}

fn main() {
    let file_path = "data/2_1.txt";
    let games = read_games_from_file(file_path).unwrap();
    println!("Solution 1: {}", solution1(&games));
    println!("Solution 2: {}", solution2(&games));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let file_path = "data/2_t1.txt";
        let games = read_games_from_file(file_path).unwrap();
        assert_eq!(solution1(&games), 8);
    }

    #[test]
    fn test_solution2() {
        let file_path = "data/2_t1.txt";
        let games = read_games_from_file(file_path).unwrap();
        assert_eq!(solution2(&games), 2286);
    }
}

use std::fs::File;
use std::io::{self, BufRead};

// Read file line by line and return reader
fn read_file(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solution1(lines: io::Lines<io::BufReader<File>>) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_BLUE: u32 = 14;
    const MAX_GREEN: u32 = 13;

    let mut res = 0;

    for line in lines {
        let line = line.unwrap();

        let (game_id, game) = line.trim_start_matches("Game ").split_once(":").unwrap();
        let mut game_valid = true;

        for round_s in game.split(";") {
            if !game_valid {
                break;
            }
            for seg in round_s.split(", ") {
                let (num, color) = seg.trim().split_once(" ").unwrap();
                let num = num.parse::<u32>().unwrap();
                match color {
                    "red" if num > MAX_RED => {
                        game_valid = false;
                        break;
                    }
                    "blue" if num > MAX_BLUE => {
                        game_valid = false;
                        break;
                    }
                    "green" if num > MAX_GREEN => {
                        game_valid = false;
                        break;
                    }
                    _ => {}
                }
            }
        }
        if game_valid {
            res += game_id.parse::<u32>().unwrap();
        }
    }
    res
}

fn solution2(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut res = 0;

    for line in lines {
        let line = line.unwrap();

        let game = line.trim_start_matches("Game ").split_once(":").unwrap().1;
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for round_s in game.split(";") {
            for seg in round_s.split(", ") {
                let (num, color) = seg.trim().split_once(" ").unwrap();
                let num = num.parse::<u32>().unwrap();
                match color {
                    "red" if num > max_red => {
                        max_red = num;
                    }
                    "blue" if num > max_blue => {
                        max_blue = num;
                    }
                    "green" if num > max_green => {
                        max_green = num;
                    }
                    _ => {}
                }
            }
        }
        res += max_red * max_blue * max_green;
    }
    res
}

fn main() {
    let sol1 = solution1(read_file("./data/2.txt").unwrap());
    println!("Solution 1: {}", sol1);

    let sol2 = solution2(read_file("./data/2.txt").unwrap());
    println!("Solution 2: {}", sol2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let sol = solution1(read_file("./data/2t.txt").unwrap());
        assert_eq!(sol, 8);
    }

    #[test]
    fn test_solution2() {
        let sol = solution2(read_file("./data/2t.txt").unwrap());
        assert_eq!(sol, 2286);
    }
}

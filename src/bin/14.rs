use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct RockPlatform {
    chars: Vec<Vec<char>>,
}

impl RockPlatform {
    fn new(input: &str) -> Self {
        let chars = input
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        return Self { chars };
    }

    fn tilt_north(&self) -> Self {
        let mut char_copy = self.chars.clone();

        for row_n in 0..self.chars[0].len() {
            let mut empty_count = 0;
            for col_n in 0..self.chars.len() {
                let char = self.chars[col_n][row_n];
                char_copy[col_n][row_n] = '.';
                match char {
                    'O' => char_copy[col_n - empty_count][row_n] = 'O',
                    '.' => empty_count += 1,
                    '#' => {
                        empty_count = 0;
                        char_copy[col_n][row_n] = '#';
                    }
                    _ => panic!("Invalid char"),
                }
            }
        }
        return Self { chars: char_copy };
    }

    fn tilt_west(&self) -> Self {
        let mut char_copy = self.chars.clone();

        for col_n in 0..self.chars.len() {
            let mut empty_count = 0;
            for row_n in 0..self.chars[0].len() {
                let char = self.chars[col_n][row_n];
                char_copy[col_n][row_n] = '.';
                match char {
                    'O' => char_copy[col_n][row_n - empty_count] = 'O',
                    '.' => empty_count += 1,
                    '#' => {
                        empty_count = 0;
                        char_copy[col_n][row_n] = '#';
                    }
                    _ => panic!("Invalid char"),
                }
            }
        }
        return Self { chars: char_copy };
    }

    fn tilt_south(&self) -> Self {
        let mut char_copy = self.chars.clone();

        for row_n in 0..self.chars[0].len() {
            let mut empty_count = 0;
            for col_n in (0..self.chars.len()).rev() {
                let char = self.chars[col_n][row_n];
                char_copy[col_n][row_n] = '.';
                match char {
                    'O' => char_copy[col_n + empty_count][row_n] = 'O',
                    '.' => empty_count += 1,
                    '#' => {
                        empty_count = 0;
                        char_copy[col_n][row_n] = '#';
                    }
                    _ => panic!("Invalid char"),
                }
            }
        }
        return Self { chars: char_copy };
    }

    fn tilt_east(&self) -> Self {
        let mut char_copy = self.chars.clone();

        for col_n in 0..self.chars.len() {
            let mut empty_count = 0;
            for row_n in (0..self.chars[0].len()).rev() {
                let char = self.chars[col_n][row_n];
                char_copy[col_n][row_n] = '.';
                match char {
                    'O' => char_copy[col_n][row_n + empty_count] = 'O',
                    '.' => empty_count += 1,
                    '#' => {
                        empty_count = 0;
                        char_copy[col_n][row_n] = '#';
                    }
                    _ => panic!("Invalid char"),
                }
            }
        }
        return Self { chars: char_copy };
    }

    fn spin(&self) -> Self {
        let mut spinned = self.tilt_north();
        spinned = spinned.tilt_west();
        spinned = spinned.tilt_south();
        spinned = spinned.tilt_east();
        return spinned;
    }

    fn calc_load(&self) -> usize {
        let mut sum = 0;
        let max_lines = self.chars.len();
        for (i, line) in self.chars.iter().enumerate() {
            for char in line {
                if *char == 'O' {
                    sum += max_lines - i;
                }
            }
        }
        return sum;
    }
}

fn get_spin_load(
    cache: &HashMap<RockPlatform, i64>,
    cycle_start: i64,
    cycle_repeat: i64,
    num_of_spins: i64,
) -> usize {
    let target_value = (num_of_spins - cycle_start) % cycle_repeat + cycle_start;
    let map = cache
        .iter()
        .find_map(|(key, &val)| if val == target_value { Some(key) } else { None })
        .unwrap();
    return map.calc_load();
}

fn solution1(inp: &str) -> usize {
    let rock_p = RockPlatform::new(inp);

    rock_p.tilt_north().calc_load()
}

fn solution2(inp: &str) -> usize {
    let mut rock_p = RockPlatform::new(inp);
    let mut cycle_start: Option<i64> = None;
    let mut cycle_repeat: Option<i64> = None;
    let mut cache = HashMap::new();

    for i in 1..1000000000i64 {
        rock_p = rock_p.spin();
        if cache.contains_key(&rock_p) {
            if cycle_start.is_none() {
                cycle_start = Some(*cache.get(&rock_p).unwrap());
                cycle_repeat = Some(i - cycle_start.unwrap());
                break;
            }
        } else {
            cache.insert(rock_p.clone(), i);
        }
    }

    return get_spin_load(
        &cache,
        cycle_start.unwrap(),
        cycle_repeat.unwrap(),
        1000000000i64,
    );
}

fn main() {
    let input = std::fs::read_to_string("./data/14.txt").unwrap();

    println!("Part 1: {}", solution1(&input));
    println!("Part 2: {}", solution2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#...."#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution1(CASE), 136);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution2(CASE), 64);
    }
}

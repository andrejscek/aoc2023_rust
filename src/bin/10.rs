#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

use Direction::*;

impl Direction {
    fn to_tuple(&self) -> (i32, i32) {
        match self {
            Right => (1, 0),
            Left => (-1, 0),
            Up => (0, -1),
            Down => (0, 1),
        }
    }

    fn move_over(&self, pos: &(i32, i32)) -> (i32, i32) {
        let d = self.to_tuple();
        (pos.0 + d.0, pos.1 + d.1)
    }

    fn next(&self, character: u8) -> Option<Direction> {
        match (self, character) {
            (Right, b'-') => Some(Right),
            (Right, b'J') => Some(Up),
            (Right, b'7') => Some(Down),
            (Left, b'-') => Some(Left),
            (Left, b'F') => Some(Down),
            (Left, b'L') => Some(Up),
            (Up, b'|') => Some(Up),
            (Up, b'F') => Some(Right),
            (Up, b'7') => Some(Left),
            (Down, b'|') => Some(Down),
            (Down, b'L') => Some(Right),
            (Down, b'J') => Some(Left),
            _ => None,
        }
    }
}

fn prep_input(input: &str) -> (Vec<&[u8]>, (i32, i32), i32, i32) {
    let mut grid = vec![];
    let mut start_pos = (0, 0);

    for (y, mut line) in input.lines().enumerate() {
        line = line.trim();
        if line.contains('S') {
            start_pos.0 = line.find('S').unwrap() as i32;
            start_pos.1 = y as i32;
        }
        grid.push(line.as_bytes());
    }
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    (grid, start_pos, width, height)
}

fn solution1(input: &str) -> usize {
    let (grid, start_pos, width, height) = prep_input(input);

    for dir in vec![Right, Left, Up, Down] {
        let mut pos = dir.move_over(&start_pos);
        if pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height {
            continue;
        }
        if dir.next(grid[pos.1 as usize][pos.0 as usize]).is_none() {
            continue;
        }

        let mut n = 1;
        let mut d = dir;
        while let Some(d_new) = d.next(grid[pos.1 as usize][pos.0 as usize]) {
            d = d_new;
            pos = d.move_over(&pos);
            n += 1;
        }
        return n / 2;
    }
    0
}

fn solution2(input: &str) -> usize {
    let (grid, start_pos, width, height) = prep_input(input);

    let mut crosses = vec![];
    for _ in 0..grid.len() {
        crosses.push(vec![]);
    }
    for dir in vec![Right, Left, Up, Down] {
        let mut pos = dir.move_over(&start_pos);
        if pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height {
            continue;
        }
        if dir.next(grid[pos.1 as usize][pos.0 as usize]).is_none() {
            continue;
        }
        if dir == Down || dir == Up {
            crosses[start_pos.1 as usize].push((start_pos.0, dir.to_tuple().1));
        }
        let mut d = dir;
        while let Some(d_new) = d.next(grid[pos.1 as usize][pos.0 as usize]) {
            if d == Down || d == Up {
                crosses[pos.1 as usize].push((pos.0, d.to_tuple().1));
            }
            if d_new == Down || d_new == Up {
                crosses[pos.1 as usize].push((pos.0, d_new.to_tuple().1));
            }
            d = d_new;
            pos = d.move_over(&pos);
        }
        if d == Down || d == Up {
            crosses[pos.1 as usize].push((pos.0, d.to_tuple().1));
        }
        break;
    }

    let mut volume = 0;
    for mut line in crosses {
        line.sort();
        let mut last = 0;
        let mut wind = 0;
        for (crosses, sign) in line {
            match (wind, sign) {
                (1, 1) | (-1, -1) => last = crosses,
                (2, -1) => volume += crosses - last - 1,
                (-2, 1) => volume -= crosses - last - 1,
                _ => (),
            }
            wind += sign;
        }
    }
    volume.abs() as usize
}

fn main() {
    let filename = "./data/10.txt";
    let input = std::fs::read_to_string(filename).unwrap();

    println!("Solution 1: {}", solution1(&input));
    println!("Solution 2: {}", solution2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE1: &str = r#".....
    .S-7.
    .|.|.
    .L-J.
    ....."#;

    const CASE2: &str = r#"..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ..."#;

    const CASE3: &str = r#"...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ..........."#;

    const CASE4: &str = r#".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ..."#;

    const CASE5: &str = r#"FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L"#;

    #[test]
    fn test_solution_1() {
        let sol1 = solution1(CASE1);
        assert_eq!(sol1, 4);

        let sol2 = solution1(CASE2);
        assert_eq!(sol2, 8);
    }

    #[test]
    fn test_solution_2() {
        let sol1 = solution2(CASE3);
        assert_eq!(sol1, 4);

        let sol2 = solution2(CASE4);
        assert_eq!(sol2, 8);

        let sol3 = solution2(CASE5);
        assert_eq!(sol3, 10);
    }
}

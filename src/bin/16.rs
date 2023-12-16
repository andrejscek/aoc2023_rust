use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Move {
    N,
    E,
    S,
    W,
}

impl Move {
    fn movement(&self) -> (isize, isize) {
        match self {
            Move::N => (0, -1),
            Move::E => (1, 0),
            Move::S => (0, 1),
            Move::W => (-1, 0),
        }
    }
}

#[derive(Debug)]
enum Tile {
    Pass,
    LeanFwd,
    LeanBck,
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let tiles = input
            .lines()
            .map(|line: &str| {
                line.trim()
                    .chars()
                    .map(|c| match c {
                        '.' => Tile::Pass,
                        '/' => Tile::LeanFwd,
                        '\\' => Tile::LeanBck,
                        '-' => Tile::Horizontal,
                        '|' => Tile::Vertical,
                        _ => panic!("Invalid char in input: '{}'", c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Grid { tiles }
    }

    fn rows(&self) -> usize {
        self.tiles[0].len()
    }

    fn cols(&self) -> usize {
        self.tiles.len()
    }

    fn do_move(&self, r: usize, c: usize, dir: Move, que: &mut VecDeque<(usize, usize, Move)>) {
        // Get movement
        let (r_add, c_add) = dir.movement();

        // Work out new position
        let new_r = r as isize + r_add;
        let new_c = c as isize + c_add;

        // Bounds check new position
        if new_r >= 0
            && (new_r as usize) < self.rows()
            && new_c >= 0
            && (new_c as usize) < self.cols()
        {
            // Within grid bounds
            que.push_back((new_r as usize, new_c as usize, dir));
        }
        // Outside grid
    }

    fn energise(&self, r: usize, c: usize, dir: Move) -> usize {
        let mut que = VecDeque::new();
        let mut visited = HashSet::new();

        que.push_back((r, c, dir)); // initial pos

        // next que pos
        while let Some((r, c, dir)) = que.pop_front() {
            // build hash set entry
            let visited_ent = (r, c, dir.clone());

            if visited.contains(&visited_ent) {
                continue;
            }

            visited.insert(visited_ent);

            match self.tiles[c][r] {
                Tile::Pass => {
                    self.do_move(r, c, dir, &mut que);
                }

                Tile::LeanFwd => {
                    // Work out new direction
                    let new_dir = match dir {
                        Move::N => Move::E,
                        Move::E => Move::N,
                        Move::S => Move::W,
                        Move::W => Move::S,
                    };
                    self.do_move(r, c, new_dir, &mut que);
                }
                Tile::LeanBck => {
                    // Work out new direction
                    let new_dir = match dir {
                        Move::N => Move::W,
                        Move::E => Move::S,
                        Move::S => Move::E,
                        Move::W => Move::N,
                    };

                    self.do_move(r, c, new_dir, &mut que);
                }
                Tile::Horizontal => match dir {
                    Move::E | Move::W => {
                        self.do_move(r, c, dir, &mut que);
                    }
                    Move::S | Move::N => {
                        // Split east and west
                        self.do_move(r, c, Move::E, &mut que);
                        self.do_move(r, c, Move::W, &mut que);
                    }
                },
                Tile::Vertical => match dir {
                    Move::S | Move::N => {
                        self.do_move(r, c, dir, &mut que);
                    }
                    Move::E | Move::W => {
                        // Split north and south
                        self.do_move(r, c, Move::N, &mut que);
                        self.do_move(r, c, Move::S, &mut que);
                    }
                },
            };
        }

        //Calculate unique visited tiles
        let visited_set = visited
            .iter()
            .map(|(r, c, _)| (r, c))
            .collect::<HashSet<_>>();

        visited_set.len()
    }
}

fn solution1(inp: &str) -> usize {
    let grid = Grid::parse(inp);
    grid.energise(0, 0, Move::E)
}

fn solution2(inp: &str) -> usize {
    let grid = Grid::parse(inp);
    let rows = grid.rows();
    let cols = grid.cols();
    let mut biggest = 0;

    for r in 0..rows {
        let energy1 = grid.energise(r, 0, Move::S);
        let energy2 = grid.energise(r, cols - 1, Move::N);
        if energy1 > biggest {
            biggest = energy1;
        }
        if energy2 > biggest {
            biggest = energy2;
        }
    }

    for c in 0..cols {
        let energy1 = grid.energise(0, c, Move::E);
        let energy2 = grid.energise(rows - 1, c, Move::W);
        if energy1 > biggest {
            biggest = energy1;
        }
        if energy2 > biggest {
            biggest = energy2;
        }
    }

    biggest
}

fn main() {
    let input = std::fs::read_to_string("./data/16.txt").unwrap();
    println!("Part 1: {}", solution1(&input));
    println!("Part 2: {}", solution2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|...."#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution1(CASE), 46);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution2(CASE), 51);
    }
}

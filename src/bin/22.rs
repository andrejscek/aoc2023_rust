use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn parse_bricks(input: &str) -> Vec<((i32, i32, i32), (i32, i32, i32))> {
    input
        .lines()
        .map(|line| {
            let (s, e) = line.trim().split_once('~').unwrap();
            let (sx, sy, sz) = s
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            let (ex, ey, ez) = e
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            ((sx, sy, sz), (ex, ey, ez))
        })
        .collect::<Vec<_>>()
}

fn get_supported(
    bricks: &mut Vec<((i32, i32, i32), (i32, i32, i32))>,
) -> Vec<((i32, i32, i32), (i32, i32, i32))> {
    bricks.sort_by(|s, e| e.0 .2.cmp(&s.0 .2));
    let mut supported_bricks = Vec::new();
    while let Some(br) = bricks.pop() {
        if br.0 .2 == 1 {
            //on floor
            supported_bricks.push(br);
        } else {
            //find first collision when falling
            let mut found_floor = true;
            supported_bricks.sort_by(|s, e| e.1 .2.cmp(&s.1 .2));

            for rb in supported_bricks.iter() {
                if rb.0 .0 > br.1 .0 || rb.1 .0 < br.0 .0 || rb.0 .1 > br.1 .1 || rb.1 .1 < br.0 .1
                {
                    //no collision, keep looking
                    continue;
                } else {
                    //collision
                    let new_z_start = rb.1 .2 + 1;
                    let new_z_stop = new_z_start + br.1 .2 - br.0 .2;
                    let new_brick = (
                        (br.0 .0, br.0 .1, new_z_start),
                        (br.1 .0, br.1 .1, new_z_stop),
                    );
                    found_floor = false;
                    supported_bricks.push(new_brick);
                    break;
                }
            }
            if found_floor {
                //found floor
                let new_brick = (
                    (br.0 .0, br.0 .1, 1),
                    (br.1 .0, br.1 .1, 1 + br.1 .2 - br.0 .2),
                );
                supported_bricks.push(new_brick);
            }
        }
    }
    supported_bricks
}

fn solution(input: &str, pt2: bool) -> i32 {
    let mut bricks = parse_bricks(input);

    //z start is always smaller than z stop
    let supported_bricks = get_supported(&mut bricks);

    let mut fixed = HashSet::new();
    let mut brick_tree = HashMap::new();

    for br in supported_bricks.clone() {
        brick_tree.insert(br, (HashSet::new(), HashSet::new()));
    }
    for br in supported_bricks.clone() {
        let mut lay_on = Vec::new();

        for rb in supported_bricks.iter() {
            if rb.0 .0 > br.1 .0 || rb.1 .0 < br.0 .0 || rb.0 .1 > br.1 .1 || rb.1 .1 < br.0 .1 {
                //no overlap
                continue;
            } else {
                //overlap, test resting
                if rb.1 .2 == br.0 .2 - 1 {
                    //resting
                    lay_on.push(rb);

                    brick_tree.get_mut(&br).unwrap().0.insert(rb);
                    brick_tree.get_mut(rb).unwrap().1.insert(br);
                }
            }
        }
        if lay_on.len() == 1 {
            fixed.insert(lay_on[0]);
        }
    }

    if !pt2 {
        return supported_bricks.len() as i32 - fixed.len() as i32;
    }

    let mut sum = 0;
    for b in brick_tree.keys() {
        let mut count = 0;
        let mut tmp_tree = brick_tree.clone();
        let mut que = VecDeque::new();
        que.push_back(b.clone());
        while let Some(br) = que.pop_front() {
            count += 1;
            for supported_b in tmp_tree.get(&br).unwrap().1.clone() {
                tmp_tree.get_mut(&supported_b).unwrap().0.remove(&br);
                if tmp_tree.get(&supported_b).unwrap().0.is_empty() {
                    que.push_back(supported_b.clone());
                }
            }
        }
        sum += count - 1;
    }
    sum
}

fn main() {
    let input = std::fs::read_to_string("./data/22.txt").unwrap();
    println!("Part 1: {}", solution(&input, false));
    println!("Part 2: {}", solution(&input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9"#;

    #[test]
    fn test_solution_1() {
        assert_eq!(solution(CASE, false), 5);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution(CASE, true), 7);
    }
}

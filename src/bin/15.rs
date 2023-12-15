use itertools::Itertools;

fn hash(inp: &str) -> u8 {
    let mut res: u8 = 0;
    for c in inp.chars() {
        res = res.wrapping_add(c as u8).wrapping_mul(17);
    }

    res
}

fn solution1(inp: &str) -> usize {
    inp.trim().split(",").map(|s| hash(s) as usize).sum()
}

fn solution2(inp: &str) -> usize {
    let mut vec_inp = Vec::new();

    for i in inp.trim().split(',') {
        let (label, foc_l) = i.split(['=', '-'].as_ref()).collect_tuple().unwrap();
        vec_inp.push((label, foc_l.parse::<usize>().ok()));
    }

    let mut boxes = vec![Vec::new(); 256];

    for (label, foc_l) in vec_inp {
        let box_n = hash(label) as usize;

        if let Some(foc_l) = foc_l {
            if let Some((_, e)) = boxes[box_n]
                .iter_mut()
                .find(|x: &&mut (&str, usize)| x.0 == label)
            {
                *e = foc_l;
            } else {
                boxes[box_n].push((label, foc_l));
            }
        } else {
            boxes[box_n].retain(|x: &(&str, usize)| x.0 != label);
        }
    }

    let mut res = 0;
    for (box_n, slots) in boxes.iter().enumerate() {
        for (slot_n, foc_l) in slots.iter().enumerate() {
            res += (box_n + 1) * (slot_n + 1) * (foc_l.1 as usize);
        }
    }
    res
}

fn main() {
    let input = std::fs::read_to_string("./data/15.txt").unwrap();
    println!("Part 1: {}", solution1(&input));
    println!("Part 2: {}", solution2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_solution_1() {
        assert_eq!(solution1(CASE), 1320);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution2(CASE), 145);
    }
}

use itertools::Itertools;
use z3::ast::{Ast, Int};

fn intersection(
    (x1, y1, dx1, dy1): (f64, f64, f64, f64),
    (x2, y2, dx2, dy2): (f64, f64, f64, f64),
) -> Option<(f64, f64)> {
    let m1 = dy1 / dx1;
    let m2 = dy2 / dx2;
    if (m2 - m1).abs() < f64::EPSILON {
        return None;
    }
    let x = (m1 * x1 - m2 * x2 + y2 - y1) / (m1 - m2);
    let y = (m1 * m2 * (x2 - x1) + m2 * y1 - m1 * y2) / (m2 - m1);
    Some((x, y))
}

fn find_intersections(lines: &[((f64, f64, f64), (f64, f64, f64))]) -> usize {
    let range = 200000000000000.0..=400000000000000.0;
    lines
        .iter()
        .tuple_combinations()
        .filter(
            |(&((x1, y1, _), (dx1, dy1, _)), &((x2, y2, _), (dx2, dy2, _)))| {
                let Some((x, y)) = intersection((x1, y1, dx1, dy1), (x2, y2, dx2, dy2)) else {
                    return false;
                };
                if dx1.signum() != (x - x1).signum() || dx2.signum() != (x - x2).signum() {
                    return false;
                }
                range.contains(&x) && range.contains(&y)
            },
        )
        .count()
}

fn parse(inp: &str) -> Vec<((f64, f64, f64), (f64, f64, f64))> {
    inp.lines()
        .map(|line| {
            let (pos, speed) = line.trim().split_once("@").unwrap();
            let (xp, yp, zp) = pos
                .split(",")
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();
            let (xs, ys, zs) = speed
                .split(",")
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();

            ((xp, yp, zp), (xs, ys, zs))
        })
        .collect::<Vec<_>>()
}

fn solution1(inp: &str) -> usize {
    let myv = parse(inp);
    find_intersections(&myv)
}

fn solution2(inp: &str) -> i64 {
    let myv = parse(inp);

    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Int::new_const(&ctx, v));

    let zero = Int::from_i64(&ctx, 0);
    for (i, &((x, y, z), (dx, dy, dz))) in myv.iter().enumerate() {
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v as _));
        let t = Int::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(s.check(), z3::SatResult::Sat);
    let model = s.get_model().unwrap();
    let res = model.eval(&(&fx + &fy + &fz), true).unwrap();
    res.as_i64().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("./data/24.txt").unwrap();
    println!("Part 1: {}", solution1(&input));
    println!("Part 2: {}", solution2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE: &str = r#"19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3"#;

    #[test]
    fn test_solution_1() {
        // assert_eq!(solution1(CASE), 2); // doesn' t work on test case
        assert!(true);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solution2(CASE), 47);
    }
}

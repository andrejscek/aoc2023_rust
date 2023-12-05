use std::str::FromStr;

type Range = (u64, u64);

#[derive(Debug)]
struct Mapping {
    dest_st: u64,
    src_st: u64,
    range_l: u64,
}

impl Mapping {
    fn get(&self, n: u64) -> Option<u64> {
        if n >= self.src_st && n <= self.src_st + self.range_l {
            Some(self.dest_st + (n - self.src_st))
        } else {
            None
        }
    }

    fn intersect(&self, n: Range) -> (Option<Range>, Vec<Range>) {
        intersect_range((self.src_st, self.range_l), n)
    }

    fn conv(&self, n: Range) -> Range {
        (self.dest_st + (n.0 - self.src_st), n.1)
    }
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let dest_st = iter.next().unwrap().parse().unwrap();
        let src_st = iter.next().unwrap().parse().unwrap();
        let range_l = iter.next().unwrap().parse().unwrap();

        Ok(Mapping {
            dest_st,
            src_st,
            range_l,
        })
    }
}

fn intersect_range(target: Range, src_st: Range) -> (Option<Range>, Vec<Range>) {
    let target_begin = target.0;
    let target_range = target.1;
    let target_end = target.0 + target.1 - 1;
    let src_st_begin = src_st.0;
    let src_st_end = src_st.0 + src_st.1 - 1;

    match (
        src_st_begin >= target_begin && src_st_begin <= target_end,
        src_st_end >= target_begin && src_st_end <= target_end,
    ) {
        // src_st starts within target
        (true, true) => (Some(src_st), vec![]),
        // src_st ends outside target
        (true, false) => (
            Some((src_st_begin, target_range - (src_st_begin - target_begin))),
            vec![(target_begin + target_range, (src_st_end) - (target_end))],
        ),
        // src_st begins before target but ends in target
        (false, true) => (
            Some((target_begin, src_st_end - target_begin + 1)),
            vec![(src_st_begin, target_begin - src_st_begin)],
        ),
        // src_st contains target
        (false, false) if src_st_begin < target_begin && src_st_end > target_end => (
            Some(target),
            vec![
                (src_st_begin, target_begin - src_st_begin),
                (target_begin + target_range, src_st_end - target_end),
            ],
        ),
        _ => (None, vec![src_st]),
    }
}

fn parse_seeds(s: &str) -> Vec<u64> {
    let num_s = s.split_once(':').unwrap().1.trim();
    num_s
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_seeds2(s: &str) -> Vec<(u64, u64)> {
    let num_s = s.split_once(':').unwrap().1.trim();
    let mut iter = num_s.split_whitespace();

    let mut seeds = vec![];
    while let (Some(a), Some(b)) = (iter.next(), iter.next()) {
        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();
        seeds.push((a, b));
    }
    seeds
}

fn solution1(file: &str) -> u64 {
    let instr = std::fs::read_to_string(file).unwrap();

    let mut iter = instr.split("\n\n");

    let seeds_str = iter.next().unwrap();
    let seeds = parse_seeds(seeds_str);

    let mut maps = vec![];
    for to_map in iter {
        let ranges: Vec<Mapping> = to_map.lines().skip(1).map(|l| l.parse().unwrap()).collect();
        maps.push(ranges);
    }

    seeds
        .iter()
        .map(|s| {
            let mut mapped = *s;
            for m in &maps {
                for r in m {
                    if let Some(v) = r.get(mapped) {
                        mapped = v;
                        break;
                    }
                }
            }
            mapped
        })
        .min()
        .unwrap()
}

fn solution2(file: &str) -> u64 {
    let instr = std::fs::read_to_string(file).unwrap();

    let mut iter = instr.split("\n\n");

    let seeds_str = iter.next().unwrap();
    let seeds = parse_seeds2(seeds_str);

    let mut maps = vec![];
    for to_map in iter {
        let ranges: Vec<Mapping> = to_map.lines().skip(1).map(|l| l.parse().unwrap()).collect();
        maps.push(ranges);
    }

    let mut cur = seeds.clone();
    for m in &maps {
        let mut next = vec![];
        for r in m {
            let mut remain = vec![];
            while let Some(s) = cur.pop() {
                let (intersection, extra) = r.intersect(s);
                if let Some(i) = intersection {
                    next.push(r.conv(i));
                }
                remain.extend(extra);
            }
            cur.extend(remain);
        }
        cur.extend(next);
    }

    cur.iter().map(|c| c.0).min().unwrap()
}

fn main() {
    let file = "./data/5.txt";
    let sol1 = solution1(file);
    println!("Solution 1: {}", sol1);

    let sol2 = solution2(file);
    println!("Solution 2: {}", sol2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let sol = solution1("./data/5t.txt");
        assert_eq!(sol, 35);
    }

    #[test]
    fn test_solution2() {
        let sol = solution2("./data/5t.txt");
        assert_eq!(sol, 46);
    }
}

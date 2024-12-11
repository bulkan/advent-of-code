use rayon::prelude::*;
use std::{iter::successors, ops::Mul};

fn run_rules(stone: &i64) -> Vec<i64> {
    if *stone == 0 {
        vec![1]
    } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
        let s = stone.to_string();
        let mid = s.len() / 2;

        let (l, r) = s.split_at(mid);

        vec![l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()]
    } else {
        vec![stone.mul(2024)]
    }
}

fn process(input: &str, run_count: usize) -> usize {
    let stones = input
        .split_whitespace()
        .map(|stone| stone.parse::<i64>().expect("should convert"))
        .collect::<Vec<_>>();

    let mut res = successors(Some(stones), |s| {
        Some(s.par_iter().flat_map(run_rules).collect::<Vec<_>>())
    });

    res.nth(run_count).unwrap().len()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = process(input, 25);
    dbg!(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "125 17";
        assert_eq!(22, process(input, 6));
    }
}

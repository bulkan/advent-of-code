use std::iter::zip;

use itertools::sorted;

fn process_list(input: &str) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in input.lines() {
        let mut line_it = line
            .split_whitespace()
            .map(|c| c.parse::<u32>().expect("should be u32"));

        let l: u32 = line_it.next().expect("should exist");
        let r: u32 = line_it.next().expect("should exist");

        left.push(l);
        right.push(r);
    }

    zip(sorted(left), sorted(right))
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");

    let total = process_list(input);

    dbg!(total);
}

#[cfg(test)]
mod tests {
    use crate::process_list;

    #[test]
    fn it_works_as_expected() {
        let input = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";

        let total = process_list(input);

        assert_eq!(total, 11);
    }
}

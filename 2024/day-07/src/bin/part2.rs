use core::panic;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::iter;

use rayon::prelude::*;

#[derive(Debug, PartialEq)]
struct TestResult {
    target: u64,
    values: Vec<u64>,
}

fn parse_test_result(input: &str) -> IResult<&str, TestResult> {
    let (input, (target, values)) = separated_pair(
        complete::u64,
        tag(": "),
        separated_list1(space1, complete::u64),
    )(input)?;

    let test_result = TestResult { target, values };

    Ok((input, test_result))
}

fn parse(input: &str) -> IResult<&str, Vec<TestResult>> {
    separated_list1(line_ending, parse_test_result)(input)
}

fn run_operations(operations: &[&str], values: Vec<u64>) -> u64 {
    let mut i = operations.iter();

    values
        .iter()
        .copied()
        .reduce(|a, b| {
            let op = i.next().expect("should exist");

            match *op {
                "||" => format!("{}{}", a, b)
                    .parse::<u64>()
                    .expect("concat should work"),
                "*" => a * b,
                "+" => a + b,
                _ => panic!("op was borked"),
            }
        })
        .unwrap()
}

fn check_test_result(test_result: &TestResult) -> bool {
    let operations = vec!["*", "+", "||"];

    let perms = iter::repeat_n(operations.into_iter(), test_result.values.len() - 1)
        .multi_cartesian_product()
        .collect::<Vec<Vec<&str>>>();

    perms
        .par_iter()
        .any(|perm| test_result.target == run_operations(perm, test_result.values.clone()))
}

fn process(input: &str) -> u64 {
    let (_, test_results) = parse(input).expect("should parse");

    test_results
        .iter()
        .filter(|test_result| check_test_result(test_result))
        .map(|test_result| test_result.target)
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = process(input);
    dbg!(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_results() {
        let test_result = TestResult {
            target: 292,
            values: vec![11, 6, 16, 20],
        };

        assert!(check_test_result(&test_result));

        let test_result = TestResult {
            target: 2,
            values: vec![1, 1],
        };

        assert!(check_test_result(&test_result));

        let test_result = TestResult {
            target: 161011,
            values: vec![16, 10, 13],
        };

        assert!(!check_test_result(&test_result));
    }

    #[test]
    fn test_run_operations() {
        assert_eq!(
            run_operations(&["+", "*", "+", "+"], vec![11, 6, 16, 20, 1]),
            293
        );
        assert_eq!(run_operations(&["+", "*", "+"], vec![11, 6, 16, 20]), 292);
        assert_eq!(run_operations(&["*", "*"], vec![1, 1, 1]), 1);
        assert_eq!(run_operations(&["+"], vec![17, 5]), 22);
        assert_eq!(run_operations(&["*"], vec![17, 5]), 85);
        assert_eq!(run_operations(&["+", "+"], vec![16, 10, 13]), 39);
        assert_eq!(run_operations(&["*", "+"], vec![13, 10, 16]), 146);
    }

    #[test]
    fn test_parse_test_result() {
        let input = "292: 11 6 16 20";

        let (_, test_result) = parse_test_result(input).expect("should work");

        let target_test_result = TestResult {
            target: 292,
            values: vec![11, 6, 16, 20],
        };

        assert_eq!(test_result, target_test_result);
    }

    #[test]
    fn test_process() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(11387, process(input));
    }
}

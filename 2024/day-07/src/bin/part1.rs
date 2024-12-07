use core::panic;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::iter;

#[derive(Debug, PartialEq)]
struct TestResult {
    target: u32,
    values: Vec<u32>,
}

// enum OPERATION {
//     MUL = "*",
//     PLUS = "+"
// }

fn parse_test_result(input: &str) -> IResult<&str, TestResult> {
    let (input, (target, values)) = separated_pair(
        complete::u32,
        tag(": "),
        separated_list1(space1, complete::u32),
    )(input)?;

    let test_result = TestResult { target, values };

    Ok((input, test_result))
}

fn parse(input: &str) -> IResult<&str, Vec<TestResult>> {
    fold_many1(
        terminated(parse_test_result, line_ending),
        Vec::default,
        |mut acc, test_result| {
            acc.push(test_result);
            acc
        },
    )(input)
}

fn run_operation(operation: &str, values: Vec<u32>) -> u32 {
    match operation {
        "*" => todo!(),
        "+" => todo!(),
        _ => panic!("unknown operation"),
    }
    0
}

fn check_test_result(test_result: &TestResult) -> bool {
    let operations = vec!["*", "+"];

    let perms = iter::repeat_n(operations.into_iter(), test_result.values.len() - 1)
        .multi_cartesian_product()
        .collect::<Vec<_>>();

    let all_mul = dbg!(perms);

    false
}

fn process(input: &str) -> u32 {
    let (_, test_results) = parse(input).expect("should parse");

    //dbg!(test_results);

    check_test_result(test_results.first().expect("should exist"));

    0
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
        assert_eq!(3749, process(input));
    }
}

use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

fn parse_rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, tag("|"), complete::u32),
            line_ending,
        ),
        Vec::default,
        |mut acc: Vec<(u32, u32)>, (a, b)| {
            acc.push((a, b));
            acc
        },
    )(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, separated_list1(tag(","), complete::u32))(input)
}

fn check_rules(update: &[u32], rules: &[(u32, u32)]) -> u32 {
    let idx = update.iter().enumerate().fold(
        HashMap::new(),
        |mut acc: HashMap<u32, usize>, (i, page)| {
            acc.insert(*page, i);
            acc
        },
    );

    for (a, b) in rules {
        if idx.contains_key(a) && idx.contains_key(b) && idx.get(a) >= idx.get(b) {
            return 0;
        }
    }

    let middle = update.get(update.len() / 2).expect("should have a middle");

    *middle
}

fn process(input: &str) -> u32 {
    let (input, rules) = terminated(parse_rules, line_ending)(input).expect("should parse rules");
    let (_input, updates) = parse_updates(input).expect("should parse the printed");

    let sum = updates
        .into_iter()
        .map(|update| check_rules(&update, &rules))
        .sum();

    dbg!(sum);

    sum
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
    fn test_process() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(143, process(input));
    }
}
